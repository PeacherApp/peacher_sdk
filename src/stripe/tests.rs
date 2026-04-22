use super::*;
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
    let mut body = CreatePaymentIntentBody::new(2000, "usd");
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
    let mut body = CreatePaymentIntentBody::new(5000, "usd");
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
