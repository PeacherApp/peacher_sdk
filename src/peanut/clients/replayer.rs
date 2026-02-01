use crate::{
    clients::{LiveClient, Transaction, TransactionError},
    prelude::*,
};
use http::StatusCode;
use std::{collections::HashMap, path::PathBuf};

pub struct Replayer<C> {
    inner: LiveClient<C>,
    transactions: HashMap<(Method, String), Vec<Transaction>>,
}

impl<C> Replayer<C> {
    pub fn wrap(client: LiveClient<C>, path: impl Into<PathBuf>) -> Self {
        tracing::warn!("Replayer was just initialized!");
        let path = path.into();
        let file = std::fs::read(path).unwrap();
        let transactions: Vec<Transaction> = serde_json::from_slice(&file).unwrap();

        let mut map: HashMap<(Method, String), Vec<Transaction>> = HashMap::new();

        for transaction in transactions {
            let method = transaction.request().method;
            let url = transaction.request().url.to_string();

            let transactions = map.entry((method, url)).or_default();

            transactions.push(transaction);
        }

        Self {
            inner: client,
            transactions: map,
        }
    }
}
impl<C: SdkConfig> Replayer<C> {
    pub async fn new(config: C, path: impl Into<PathBuf>) -> SdkResult<Self> {
        Ok(Self::wrap(LiveClient::new(config).await?, path))
    }
    pub async fn new_with_client(
        client: reqwest::Client,
        config: C,
        path: impl Into<PathBuf>,
    ) -> SdkResult<Self> {
        Ok(Self::wrap(
            LiveClient::new_with_client(client, config).await?,
            path,
        ))
    }
}

impl<C: SdkConfig> SdkClient for Replayer<C> {
    type Sdk = C;
    async fn request<H>(&self, handler: &H) -> SdkResult<H::ResponseBody>
    where
        H: SdkRouteHandler<Sdk = Self::Sdk>,
    {
        let method = handler.method();
        let request = self.inner.inner_request(handler);
        let url = request.url();

        let key = (method, url.to_string());

        let Some(transactions) = self.transactions.get(&key) else {
            panic!("Couldn't find key for {method:?} {url}");
        };
        let transaction = transactions.first().unwrap();

        let Some(response) = transaction.response() else {
            return Err(SdkError::Request(None));
        };

        match &response.body {
            Ok(body) => match serde_json::from_value(body.clone()) {
                Ok(val) => Ok(val),
                Err(e) => Err(SdkError::DeserializeValue(e)),
            },
            Err(TransactionError::InvalidBody) => Err(SdkError::DeserializeBody(None)),
            Err(TransactionError::Status) => Err(SdkError::Status(
                StatusCode::from_u16(response.status).unwrap(),
                None,
            )),
        }
    }
}

// pub struct Replayer {
//     client: reqwest::Client,
//     transactions: HashMap<(Method, String), Vec<Transaction>>,
//     config: DefaultSdkConfig,
// }

// impl Replayer {
//     pub fn new(
//         client: reqwest::Client,
//         path: impl Into<PathBuf>,
//         config: DefaultSdkConfig,
//     ) -> Self {
//         tracing::warn!("Replayer was just initialized!");
//         let path = path.into();
//         let file = std::fs::read(path).unwrap();
//         let transactions: Vec<Transaction> = serde_json::from_slice(&file).unwrap();

//         let mut map: HashMap<(Method, String), Vec<Transaction>> = HashMap::new();

//         for transaction in transactions {
//             let method = transaction.request().method;
//             let url = transaction.request().url.to_string();

//             let transactions = map.entry((method, url)).or_default();

//             transactions.push(transaction);
//         }

//         Self {
//             client,
//             transactions: map,
//             config,
//         }
//     }
// }

// impl OldClientThing for Replayer {
//     const FAKES_REQUESTS: bool = true;
//     async fn raw_external_request<W: RespondWith>(
//         &mut self,
//         method: Method,
//         passed_url: impl IntoUrl + Send,
//         respond_with: W,
//         params: impl FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder + Send,
//     ) -> Result<W::Output, super::SdkError> {
//         let request = params(self.client.request(method.to_reqwest_method(), passed_url))
//             .build()
//             .unwrap();
//         let url = request.url();

//         let key = (method, url.to_string());

//         let Some(transactions) = self.transactions.get(&key) else {
//             panic!("Couldn't find key for {method:?} {url}");
//         };
//         let transaction = transactions.first().unwrap();

//         let Some(response) = transaction.response() else {
//             return Err(SdkError::Request(None));
//         };

//         match &response.body {
//             Ok(body) => respond_with.handle_save(body.clone()),
//             Err(TransactionError::InvalidBody) => Err(SdkError::DeserializeBody(None)),
//             Err(TransactionError::Status) => Err(SdkError::Status(
//                 StatusCode::from_u16(response.status).unwrap(),
//                 None,
//             )),
//         }
//     }

//     fn config(&self) -> &super::DefaultSdkConfig {
//         &self.config
//     }
// }
