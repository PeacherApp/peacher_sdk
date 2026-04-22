mod payment_intent;
pub use payment_intent::*;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePaymentIntent {}

impl Handler for CreatePaymentIntent {
    type ResponseBody = PaymentIntent;
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/v1/payment_intents".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ahash::HashMapExt;

    #[test]
    fn deserializes_stripe_example_payload() {
        let json = serde_json::json!({
            "id": "pi_3MtwBwLkdIwHu7ix28a3tqPa",
            "object": "payment_intent",
            "amount": 2000,
            "amount_capturable": 0,
            "amount_details": {
                "tip": {}
            },
            "amount_received": 0,
            "application": null,
            "application_fee_amount": null,
            "automatic_payment_methods": {
                "enabled": true
            },
            "canceled_at": null,
            "cancellation_reason": null,
            "capture_method": "automatic",
            "client_secret": "pi_3MtwBwLkdIwHu7ix28a3tqPa_secret_YrKJUKribcBjcG8HVhfZluoGH",
            "confirmation_method": "automatic",
            "created": 1680800504,
            "currency": "usd",
            "customer": null,
            "description": null,
            "last_payment_error": null,
            "latest_charge": null,
            "livemode": false,
            "metadata": {},
            "next_action": null,
            "on_behalf_of": null,
            "payment_method": null,
            "payment_method_options": {
                "card": {
                    "installments": null,
                    "mandate_options": null,
                    "network": null,
                    "request_three_d_secure": "automatic"
                },
                "link": {
                    "persistent_token": null
                }
            },
            "payment_method_types": ["card", "link"],
            "processing": null,
            "receipt_email": null,
            "review": null,
            "setup_future_usage": null,
            "shipping": null,
            "source": null,
            "statement_descriptor": null,
            "statement_descriptor_suffix": null,
            "status": "requires_payment_method",
            "transfer_data": null,
            "transfer_group": null
        });

        let intent: PaymentIntent = serde_json::from_value(json).expect("should deserialize");
        assert_eq!(intent.id, "pi_3MtwBwLkdIwHu7ix28a3tqPa");
        assert_eq!(intent.amount, 2000);
        assert_eq!(intent.capture_method, CaptureMethod::Automatic);
        assert_eq!(intent.confirmation_method, ConfirmationMethod::Automatic);
        assert_eq!(intent.status, PaymentIntentStatus::RequiresPaymentMethod);
        assert_eq!(intent.payment_method_types, vec!["card", "link"]);
        assert!(intent.automatic_payment_methods.unwrap().enabled);
        let card = intent
            .payment_method_options
            .as_ref()
            .unwrap()
            .card
            .as_ref()
            .unwrap();
        assert_eq!(
            card.request_three_d_secure,
            Some(RequestThreeDSecure::Automatic)
        );
    }

    #[test]
    fn serializes_with_skipped_none_fields() {
        let intent = PaymentIntent {
            id: "pi_123".into(),
            object: "payment_intent".into(),
            amount: 1000,
            amount_capturable: 0,
            amount_details: None,
            amount_received: 0,
            application: None,
            application_fee_amount: None,
            automatic_payment_methods: None,
            canceled_at: None,
            cancellation_reason: None,
            capture_method: CaptureMethod::AutomaticAsync,
            client_secret: None,
            confirmation_method: ConfirmationMethod::Automatic,
            created: 0,
            currency: "usd".into(),
            customer: None,
            customer_account: None,
            description: None,
            excluded_payment_method_types: None,
            hooks: None,
            last_payment_error: None,
            latest_charge: None,
            livemode: false,
            managed_payments: None,
            metadata: HashMap::default(),
            next_action: None,
            on_behalf_of: None,
            payment_details: None,
            payment_method: None,
            payment_method_configuration_details: None,
            payment_method_options: None,
            payment_method_types: vec!["card".into()],
            presentment_details: None,
            processing: None,
            receipt_email: None,
            review: None,
            setup_future_usage: None,
            shipping: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            status: PaymentIntentStatus::RequiresPaymentMethod,
            transfer_data: None,
            transfer_group: None,
        };

        let serialized = serde_json::to_value(&intent).unwrap();
        assert!(!serialized.as_object().unwrap().contains_key("description"));
        assert_eq!(serialized["capture_method"], "automatic_async");
    }
}
