//! Stripe [Event](https://docs.stripe.com/api/events) types for webhook handlers.
//!
//! The shape is [`WebhookEvent`] with a strongly-typed [`WebhookEventKind`] discriminated
//! by Stripe's top-level `type` field. Event types we haven't modeled yet are preserved
//! as [`WebhookEventKind::Unknown`] with the raw object JSON so forward-compatibility
//! never forces a deploy.

use serde::Deserialize;
use serde::de::{self, Deserializer};

use crate::stripe::{CheckoutSession, PaymentIntent};

/// A Stripe webhook event.
///
/// See: <https://docs.stripe.com/api/events/object>
#[derive(Debug, Clone)]
pub struct WebhookEvent {
    pub id: String,
    pub api_version: Option<String>,
    pub created: i64,
    pub livemode: bool,
    pub pending_webhooks: i32,
    pub request: Option<WebhookRequest>,
    /// The typed payload, dispatched on the event's `type` field.
    pub kind: WebhookEventKind,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WebhookRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

/// The typed payload of a webhook event.
///
/// Add a variant here when you start handling a new event type. Everything not
/// explicitly matched falls through to [`WebhookEventKind::Unknown`] — callers
/// that only care about a subset of events can match on that subset and
/// discard the rest.
#[derive(Debug, Clone)]
pub enum WebhookEventKind {
    CheckoutSessionCompleted(CheckoutSession),
    CheckoutSessionExpired(CheckoutSession),
    CheckoutSessionAsyncPaymentFailed(CheckoutSession),
    CheckoutSessionAsyncPaymentSucceeded(CheckoutSession),
    PaymentIntentCreated(PaymentIntent),
    PaymentIntentSucceeded(PaymentIntent),
    PaymentIntentPaymentFailed(PaymentIntent),
    PaymentIntentCanceled(PaymentIntent),
    PaymentIntentProcessing(PaymentIntent),
    PaymentIntentRequiresAction(PaymentIntent),
    /// An event type we don't model as a typed variant yet.
    Unknown {
        type_name: String,
        object: serde_json::Value,
    },
}

impl WebhookEventKind {
    /// The raw Stripe `type` string for this event (e.g. `"checkout.session.completed"`).
    pub fn type_name(&self) -> &str {
        match self {
            Self::CheckoutSessionCompleted(_) => "checkout.session.completed",
            Self::CheckoutSessionExpired(_) => "checkout.session.expired",
            Self::CheckoutSessionAsyncPaymentFailed(_) => "checkout.session.async_payment_failed",
            Self::CheckoutSessionAsyncPaymentSucceeded(_) => {
                "checkout.session.async_payment_succeeded"
            }
            Self::PaymentIntentCreated(_) => "payment_intent.created",
            Self::PaymentIntentSucceeded(_) => "payment_intent.succeeded",
            Self::PaymentIntentPaymentFailed(_) => "payment_intent.payment_failed",
            Self::PaymentIntentCanceled(_) => "payment_intent.canceled",
            Self::PaymentIntentProcessing(_) => "payment_intent.processing",
            Self::PaymentIntentRequiresAction(_) => "payment_intent.requires_action",
            Self::Unknown { type_name, .. } => type_name,
        }
    }
}

// Deserialization lives in one place so the `type` dispatch stays co-located
// with the enum it fills in. Serialization isn't needed — webhook payloads
// are only ever consumed, never produced by our code.
impl<'de> Deserialize<'de> for WebhookEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            id: String,
            #[serde(default)]
            api_version: Option<String>,
            created: i64,
            livemode: bool,
            #[serde(default)]
            pending_webhooks: i32,
            #[serde(default)]
            request: Option<WebhookRequest>,
            #[serde(rename = "type")]
            event_type: String,
            data: RawData,
        }

        #[derive(Deserialize)]
        struct RawData {
            object: serde_json::Value,
        }

        let raw = Raw::deserialize(deserializer)?;

        // Helper to deserialize the object into the expected resource type and wrap it
        // in a variant. Errors bubble up with context about which event type we tried.
        fn typed<T, E, F>(
            event_type: &str,
            object: serde_json::Value,
            wrap: F,
        ) -> Result<WebhookEventKind, E>
        where
            T: for<'a> Deserialize<'a>,
            E: de::Error,
            F: FnOnce(T) -> WebhookEventKind,
        {
            serde_json::from_value::<T>(object).map(wrap).map_err(|e| {
                E::custom(format!(
                    "failed to deserialize `{}` payload: {}",
                    event_type, e
                ))
            })
        }

        let kind = match raw.event_type.as_str() {
            "checkout.session.completed" => typed::<CheckoutSession, D::Error, _>(
                &raw.event_type,
                raw.data.object,
                WebhookEventKind::CheckoutSessionCompleted,
            )?,
            "checkout.session.expired" => typed::<CheckoutSession, D::Error, _>(
                &raw.event_type,
                raw.data.object,
                WebhookEventKind::CheckoutSessionExpired,
            )?,
            "checkout.session.async_payment_failed" => typed::<CheckoutSession, D::Error, _>(
                &raw.event_type,
                raw.data.object,
                WebhookEventKind::CheckoutSessionAsyncPaymentFailed,
            )?,
            "checkout.session.async_payment_succeeded" => typed::<CheckoutSession, D::Error, _>(
                &raw.event_type,
                raw.data.object,
                WebhookEventKind::CheckoutSessionAsyncPaymentSucceeded,
            )?,
            "payment_intent.created" => typed::<PaymentIntent, D::Error, _>(
                &raw.event_type,
                raw.data.object,
                WebhookEventKind::PaymentIntentCreated,
            )?,
            "payment_intent.succeeded" => typed::<PaymentIntent, D::Error, _>(
                &raw.event_type,
                raw.data.object,
                WebhookEventKind::PaymentIntentSucceeded,
            )?,
            "payment_intent.payment_failed" => typed::<PaymentIntent, D::Error, _>(
                &raw.event_type,
                raw.data.object,
                WebhookEventKind::PaymentIntentPaymentFailed,
            )?,
            "payment_intent.canceled" => typed::<PaymentIntent, D::Error, _>(
                &raw.event_type,
                raw.data.object,
                WebhookEventKind::PaymentIntentCanceled,
            )?,
            "payment_intent.processing" => typed::<PaymentIntent, D::Error, _>(
                &raw.event_type,
                raw.data.object,
                WebhookEventKind::PaymentIntentProcessing,
            )?,
            "payment_intent.requires_action" => typed::<PaymentIntent, D::Error, _>(
                &raw.event_type,
                raw.data.object,
                WebhookEventKind::PaymentIntentRequiresAction,
            )?,
            other => WebhookEventKind::Unknown {
                type_name: other.to_string(),
                object: raw.data.object,
            },
        };

        Ok(WebhookEvent {
            id: raw.id,
            api_version: raw.api_version,
            created: raw.created,
            livemode: raw.livemode,
            pending_webhooks: raw.pending_webhooks,
            request: raw.request,
            kind,
        })
    }
}
