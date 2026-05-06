use super::*;
use crate::prelude::{GetHandler, Handler, Method};
use ahash::HashMap;

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

#[test]
fn create_body_matches_stripe_curl_example() {
    // Mirrors the docs example:
    //   -d amount=2000
    //   -d currency=usd
    //   -d "automatic_payment_methods[enabled]=true"
    let mut body = CreatePaymentIntent::new(2000, "usd");
    body.automatic_payment_methods = Some(CreateAutomaticPaymentMethods {
        enabled: true,
        allow_redirects: None,
    });

    let encoded = serde_qs::to_string(&body).expect("serializes as form-qs");

    assert!(encoded.contains("amount=2000"), "got: {encoded}");
    assert!(encoded.contains("currency=usd"), "got: {encoded}");
    assert!(
        encoded.contains("automatic_payment_methods[enabled]=true"),
        "got: {encoded}"
    );

    // Fields we never set must not leak into the body.
    assert!(!encoded.contains("description"), "got: {encoded}");
    assert!(!encoded.contains("capture_method"), "got: {encoded}");
    assert!(!encoded.contains("confirm="), "got: {encoded}");
}

#[test]
fn create_body_nests_payment_method_data_under_type_key() {
    let mut body = CreatePaymentIntent::new(5000, "usd");
    body.payment_method_data = Some(CreatePaymentMethodData {
        data_type: PaymentMethodType::SepaDebit,
        sepa_debit: Some(CreatePaymentMethodDataSepaDebit {
            iban: "DE89370400440532013000".into(),
        }),
        acss_debit: None,
        affirm: None,
        afterpay_clearpay: None,
        alipay: None,
        allow_redisplay: None,
        alma: None,
        amazon_pay: None,
        au_becs_debit: None,
        bacs_debit: None,
        bancontact: None,
        billie: None,
        billing_details: None,
        blik: None,
        boleto: None,
        cashapp: None,
        crypto: None,
        customer_balance: None,
        eps: None,
        fpx: None,
        giropay: None,
        grabpay: None,
        ideal: None,
        interac_present: None,
        kakao_pay: None,
        klarna: None,
        konbini: None,
        kr_card: None,
        link: None,
        mb_way: None,
        metadata: HashMap::default(),
        mobilepay: None,
        multibanco: None,
        naver_pay: None,
        nz_bank_account: None,
        oxxo: None,
        p24: None,
        pay_by_bank: None,
        payco: None,
        paynow: None,
        paypal: None,
        paypay: None,
        payto: None,
        pix: None,
        promptpay: None,
        radar_options: None,
        revolut_pay: None,
        samsung_pay: None,
        satispay: None,
        sofort: None,
        sunbit: None,
        swish: None,
        twint: None,
        upi: None,
        us_bank_account: None,
        wechat_pay: None,
        zip: None,
    });

    let encoded = serde_qs::to_string(&body).expect("serializes as form-qs");
    assert!(
        encoded.contains("payment_method_data[type]=sepa_debit"),
        "got: {encoded}"
    );
    assert!(
        encoded.contains("payment_method_data[sepa_debit][iban]=DE89370400440532013000"),
        "got: {encoded}"
    );
}

#[test]
fn deserializes_checkout_session_example_payload() {
    let raw = r#"{
        "id": "cs_test_a11YYufWQzNY63zpQ6QSNRQhkUpVph4WRmzW0zWJO2znZKdVujZ0N0S22u",
        "object": "checkout.session",
        "after_expiration": null,
        "allow_promotion_codes": null,
        "amount_subtotal": 2198,
        "amount_total": 2198,
        "automatic_tax": {
            "enabled": false,
            "liability": null,
            "status": null
        },
        "billing_address_collection": null,
        "cancel_url": null,
        "client_reference_id": null,
        "consent": null,
        "consent_collection": null,
        "created": 1679600215,
        "currency": "usd",
        "custom_fields": [],
        "custom_text": {
            "shipping_address": null,
            "submit": null
        },
        "customer": null,
        "customer_creation": "if_required",
        "customer_details": null,
        "customer_email": null,
        "expires_at": 1679686615,
        "invoice": null,
        "invoice_creation": {
            "enabled": false,
            "invoice_data": {
                "account_tax_ids": null,
                "custom_fields": null,
                "description": null,
                "footer": null,
                "issuer": null,
                "metadata": {},
                "rendering_options": null
            }
        },
        "livemode": false,
        "locale": null,
        "metadata": {},
        "mode": "payment",
        "payment_intent": null,
        "payment_link": null,
        "payment_method_collection": "always",
        "payment_method_options": {},
        "payment_method_types": ["card"],
        "payment_status": "unpaid",
        "phone_number_collection": {
            "enabled": false
        },
        "recovered_from": null,
        "setup_intent": null,
        "shipping_address_collection": null,
        "shipping_cost": null,
        "shipping_details": null,
        "shipping_options": [],
        "status": "open",
        "submit_type": null,
        "subscription": null,
        "success_url": "https://example.com/success",
        "total_details": {
            "amount_discount": 0,
            "amount_shipping": 0,
            "amount_tax": 0
        },
        "url": "https://checkout.stripe.com/c/pay/cs_test_a11YYufWQzNY63zpQ6QSNRQhkUpVph4WRmzW0zWJO2znZKdVujZ0N0S22u"
    }"#;

    let session: CheckoutSession = serde_json::from_str(raw).expect("should deserialize");
    assert_eq!(
        session.id,
        "cs_test_a11YYufWQzNY63zpQ6QSNRQhkUpVph4WRmzW0zWJO2znZKdVujZ0N0S22u"
    );
    assert_eq!(session.amount_total, Some(2198));
    assert_eq!(session.amount_subtotal, Some(2198));
    assert_eq!(session.currency, Some("usd".to_string()));
    assert_eq!(session.mode, CheckoutSessionMode::Payment);
    assert_eq!(session.status, CheckoutSessionStatus::Open);
    assert_eq!(session.payment_status, CheckoutPaymentStatus::Unpaid);
    assert_eq!(
        session.payment_method_collection,
        Some(PaymentMethodCollection::Always)
    );
    assert_eq!(
        session.customer_creation,
        Some(CheckoutCustomerCreation::IfRequired)
    );
    assert_eq!(session.payment_method_types, vec!["card".to_string()]);
    assert!(!session.automatic_tax.enabled);

    let totals = session.total_details.expect("total_details present");
    assert_eq!(totals.amount_discount, 0);
    assert_eq!(totals.amount_shipping, 0);
    assert_eq!(totals.amount_tax, 0);

    let invoice_creation = session.invoice_creation.expect("invoice_creation present");
    assert!(!invoice_creation.enabled);
}

#[test]
fn create_checkout_session_matches_stripe_curl_example() {
    // Mirrors the docs example:
    //   --data-urlencode "success_url=https://example.com/success"
    //   -d "line_items[0][price]={{PRICE_ID}}"
    //   -d "line_items[0][quantity]=2"
    //   -d mode=payment
    let mut body = CreateCheckoutSession::new(CheckoutSessionMode::Payment);
    body.success_url = Some("https://example.com/success".into());
    body.line_items = Some(vec![CreateLineItem {
        adjustable_quantity: None,
        dynamic_tax_rates: None,
        metadata: HashMap::default(),
        price: Some("price_123".into()),
        price_data: None,
        quantity: Some(2),
        tax_rates: None,
    }]);

    let encoded = serde_qs::to_string(&body).expect("serializes as form-qs");

    assert!(encoded.contains("mode=payment"), "got: {encoded}");
    assert!(
        encoded.contains("success_url=https://example.com/success")
            || encoded.contains("success_url=https%3A%2F%2Fexample.com%2Fsuccess"),
        "got: {encoded}"
    );
    assert!(
        encoded.contains("line_items[0][price]=price_123"),
        "got: {encoded}"
    );
    assert!(
        encoded.contains("line_items[0][quantity]=2"),
        "got: {encoded}"
    );

    // Fields we never set must not leak into the body.
    assert!(!encoded.contains("cancel_url"), "got: {encoded}");
    assert!(!encoded.contains("payment_intent_data"), "got: {encoded}");
}

#[test]
fn create_checkout_session_nests_subscription_data() {
    let mut body = CreateCheckoutSession::new(CheckoutSessionMode::Subscription);
    body.subscription_data = Some(CreateSubscriptionData {
        trial_period_days: Some(14),
        description: Some("monthly plan".into()),
        ..Default::default()
    });

    let encoded = serde_qs::to_string(&body).expect("serializes as form-qs");
    assert!(
        encoded.contains("subscription_data[trial_period_days]=14"),
        "got: {encoded}"
    );
    assert!(
        encoded.contains("subscription_data[description]=monthly%20plan")
            || encoded.contains("subscription_data[description]=monthly+plan"),
        "got: {encoded}"
    );
}

#[test]
fn retrieve_and_expire_checkout_session_paths() {
    let retrieve = RetrieveCheckoutSession::new("cs_test_abc");
    assert_eq!(
        GetHandler::path(&retrieve),
        "/v1/checkout/sessions/cs_test_abc"
    );

    let expire = ExpireCheckoutSession::new("cs_test_abc");
    assert_eq!(expire.path(), "/v1/checkout/sessions/cs_test_abc/expire");
    assert_eq!(expire.method(), Method::Post);
}

#[test]
fn list_checkout_session_line_items_path_has_id() {
    let list = ListCheckoutSessionLineItems::new("cs_test_123");
    assert_eq!(
        GetHandler::path(&list),
        "/v1/checkout/sessions/cs_test_123/line_items"
    );
}

#[test]
fn webhook_event_deserializes_checkout_session_completed() {
    let raw = r#"{
        "id": "evt_1PXYZ",
        "object": "event",
        "api_version": "2020-08-27",
        "created": 1679600215,
        "data": {
            "object": {
                "id": "cs_test_abc",
                "object": "checkout.session",
                "amount_subtotal": 2000,
                "amount_total": 2000,
                "automatic_tax": { "enabled": false, "liability": null, "status": null },
                "created": 1679600000,
                "currency": "usd",
                "custom_fields": [],
                "livemode": false,
                "metadata": {},
                "mode": "payment",
                "payment_intent": "pi_test_123",
                "payment_method_types": ["card"],
                "payment_status": "paid",
                "status": "complete",
                "success_url": "https://example.com/success",
                "url": "https://checkout.stripe.com/c/pay/cs_test_abc"
            }
        },
        "livemode": false,
        "pending_webhooks": 1,
        "request": { "id": null, "idempotency_key": null },
        "type": "checkout.session.completed"
    }"#;

    let event: WebhookEvent = serde_json::from_str(raw).expect("should deserialize");
    assert_eq!(event.id, "evt_1PXYZ");
    assert_eq!(event.api_version.as_deref(), Some("2020-08-27"));
    assert_eq!(event.created, 1679600215);
    assert!(!event.livemode);
    assert_eq!(event.kind.type_name(), "checkout.session.completed");

    match event.kind {
        WebhookEventKind::CheckoutSessionCompleted(session) => {
            assert_eq!(session.id, "cs_test_abc");
            assert_eq!(session.payment_intent.as_deref(), Some("pi_test_123"));
            assert_eq!(session.payment_status, CheckoutPaymentStatus::Paid);
            assert_eq!(session.status, CheckoutSessionStatus::Complete);
        }
        other => panic!("expected CheckoutSessionCompleted, got {:?}", other),
    }
}

#[test]
fn webhook_event_deserializes_payment_intent_payment_failed() {
    let raw = r#"{
        "id": "evt_failed",
        "object": "event",
        "created": 1679600215,
        "data": {
            "object": {
                "id": "pi_test_fail",
                "object": "payment_intent",
                "amount": 5000,
                "amount_capturable": 0,
                "amount_received": 0,
                "capture_method": "automatic",
                "confirmation_method": "automatic",
                "created": 1679600000,
                "currency": "usd",
                "last_payment_error": {
                    "code": "card_declined",
                    "message": "Your card was declined.",
                    "type": "card_error"
                },
                "livemode": false,
                "metadata": {},
                "payment_method_types": ["card"],
                "status": "requires_payment_method"
            }
        },
        "livemode": false,
        "pending_webhooks": 1,
        "type": "payment_intent.payment_failed"
    }"#;

    let event: WebhookEvent = serde_json::from_str(raw).expect("should deserialize");
    match event.kind {
        WebhookEventKind::PaymentIntentPaymentFailed(intent) => {
            assert_eq!(intent.id, "pi_test_fail");
            let err = intent.last_payment_error.expect("has last_payment_error");
            assert_eq!(err.code.as_deref(), Some("card_declined"));
            assert_eq!(err.error_type, ErrorType::CardError);
        }
        other => panic!("expected PaymentIntentPaymentFailed, got {:?}", other),
    }
}

#[test]
fn webhook_event_preserves_unknown_event_type_as_raw_object() {
    let raw = r#"{
        "id": "evt_other",
        "object": "event",
        "created": 1679600215,
        "data": { "object": { "id": "ch_test", "object": "charge" } },
        "livemode": false,
        "pending_webhooks": 0,
        "type": "charge.refunded"
    }"#;

    let event: WebhookEvent = serde_json::from_str(raw).expect("should deserialize");
    match event.kind {
        WebhookEventKind::Unknown { type_name, object } => {
            assert_eq!(type_name, "charge.refunded");
            assert_eq!(object["id"].as_str(), Some("ch_test"));
        }
        other => panic!("expected Unknown, got {:?}", other),
    }
}

#[test]
fn webhook_event_surfaces_nested_parse_errors() {
    // `mode` is required on CheckoutSession — missing it should bubble up a
    // descriptive error rather than silently falling back to Unknown.
    let raw = r#"{
        "id": "evt_broken",
        "object": "event",
        "created": 1,
        "data": { "object": { "id": "cs_x", "object": "checkout.session" } },
        "livemode": false,
        "pending_webhooks": 0,
        "type": "checkout.session.completed"
    }"#;

    let err = serde_json::from_str::<WebhookEvent>(raw)
        .expect_err("missing required CheckoutSession fields should fail");
    let msg = err.to_string();
    assert!(
        msg.contains("checkout.session.completed"),
        "error should name the event type, got: {msg}"
    );
}
