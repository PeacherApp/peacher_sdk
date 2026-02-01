use std::{fs::File, io::Write, path::PathBuf, sync::Mutex};

use crate::{
    clients::{
        LiveClient, Transaction, TransactionError, TransactionHeaders, TransactionRequest,
        TransactionResponse,
    },
    prelude::*,
};

pub struct Saver<C> {
    inner: LiveClient<C>,
    file: File,
    transactions: Mutex<Vec<Transaction>>,
}

impl<C> Saver<C> {
    pub fn wrap(client: LiveClient<C>, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let file = std::fs::File::create(path).unwrap();
        Self {
            inner: client,
            file,
            transactions: Mutex::new(Vec::new()),
        }
    }

    pub fn save(mut self) {
        let transactions = serde_json::to_vec(&self.transactions).unwrap();
        self.file.write_all(&transactions).unwrap();
        self.file.flush().unwrap();
    }
}
impl<C: SdkConfig> Saver<C> {
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

impl<C: SdkConfig> SdkClient for Saver<C> {
    type Sdk = C;
    async fn request<H>(&self, handler: &H) -> SdkResult<H::ResponseBody>
    where
        H: SdkRouteHandler<Sdk = Self::Sdk>,
    {
        let url = handler.url(self.inner.config());
        let method = handler.method();

        let request = self.inner.inner_request(handler);
        let transaction_request = TransactionRequest {
            method,
            url,
            headers: TransactionHeaders::from_headermap(request.headers()),
        };

        let response = match self.inner.client().execute(request).await {
            Ok(response) => response,
            Err(e) => {
                let mut lock = self.transactions.lock().unwrap();
                lock.push(Transaction::req_err(transaction_request));

                return Err(SdkError::Request(Some(e)));
            }
        };

        let response_status = response.status().as_u16();
        let response_headers = TransactionHeaders::from_headermap(response.headers());

        if response.status().is_client_error() || response.status().is_server_error() {
            tracing::error!("Error response: {:?}", response.status());
            let mut lock = self.transactions.lock().unwrap();
            lock.push(Transaction::new(
                transaction_request,
                TransactionResponse::new_err(
                    response_status,
                    response_headers,
                    TransactionError::Status,
                ),
            ));

            return Err(SdkError::Status(
                response.status(),
                Some(Box::new(response)),
            ));
        }

        match self.inner.inner_response(handler, response).await {
            Ok(body) => {
                let mut lock = self.transactions.lock().unwrap();

                let val = serde_json::to_value(body.clone()).unwrap();

                lock.push(Transaction::new(
                    transaction_request,
                    TransactionResponse::new(response_status, response_headers, val),
                ));

                Ok(body)
            }
            Err(e) => {
                let mut lock = self.transactions.lock().unwrap();
                lock.push(Transaction::new(
                    transaction_request,
                    TransactionResponse::new_err(
                        response_status,
                        response_headers,
                        TransactionError::Status,
                    ),
                ));

                Err(e)
            }
        }
    }
}

// pub struct Saver {
//     client: reqwest::Client,
//     file: File,
//     config: DefaultSdkConfig,
//     transactions: Vec<Transaction>,
// }

// impl Saver {
//     /// path to save
//     pub fn new(
//         client: reqwest::Client,
//         path: impl Into<PathBuf>,
//         config: DefaultSdkConfig,
//     ) -> Self {
//         let path = path.into();
//         let file = std::fs::File::create(path).unwrap();

//         Self {
//             client,
//             file,
//             config,
//             transactions: Vec::new(),
//         }
//     }

//     pub fn save(mut self) {
//         let transactions = serde_json::to_vec(&self.transactions).unwrap();
//         self.file.write_all(&transactions).unwrap();
//         self.file.flush().unwrap();
//     }
// }
// impl OldClientThing for Saver {
//     const FAKES_REQUESTS: bool = false;
//     async fn raw_external_request<W: RespondWith>(
//         &mut self,
//         method: Method,
//         url: impl IntoUrl + Send,
//         respond_with: W,
//         params: impl FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder + Send,
//     ) -> Result<W::Output, super::SdkError> {
//         let request = params(self.client.request(method.to_reqwest_method(), url))
//             .build()
//             .unwrap();
//         let url = request.url().clone();

//         let transaction_request = TransactionRequest {
//             method,
//             url,
//             headers: TransactionHeaders::from_headermap(request.headers()),
//         };
//         let response = match self.client.execute(request).await {
//             Ok(response) => response,
//             Err(e) => {
//                 self.transactions
//                     .push(Transaction::req_err(transaction_request));

//                 return Err(SdkError::Request(Some(e)));
//             }
//         };

//         let response_status = response.status().as_u16();
//         let response_headers = TransactionHeaders::from_headermap(response.headers());

//         if response.status().is_client_error() || response.status().is_server_error() {
//             tracing::error!("Error response: {:?}", response.status());
//             self.transactions.push(Transaction::new(
//                 transaction_request,
//                 TransactionResponse::new_err(
//                     response_status,
//                     response_headers,
//                     TransactionError::Status,
//                 ),
//             ));

//             return Err(SdkError::Status(
//                 response.status(),
//                 Some(Box::new(response)),
//             ));
//         }

//         match respond_with.handle_response(response).await {
//             Ok(result) => {
//                 self.transactions.push(Transaction::new(
//                     transaction_request,
//                     TransactionResponse::new(
//                         response_status,
//                         response_headers,
//                         result.to_saved_response(),
//                     ),
//                 ));
//                 Ok(result)
//             }
//             Err(e) => {
//                 self.transactions.push(Transaction::new(
//                     transaction_request,
//                     TransactionResponse::new_err(
//                         response_status,
//                         response_headers,
//                         TransactionError::InvalidBody,
//                     ),
//                 ));
//                 Err(e)
//             }
//         }
//     }

//     fn config(&self) -> &super::DefaultSdkConfig {
//         &self.config
//     }
// }
