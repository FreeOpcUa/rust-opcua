mod client;
mod connect;
mod connection;
mod event_loop;
mod implementation;
mod request_builder;
mod retry;
mod services;

/// Information about the server endpoint, security policy, security mode and user identity that the session will
/// will use to establish a connection.
#[derive(Debug, Clone)]
pub struct SessionInfo {
    /// The endpoint
    pub endpoint: EndpointDescription,
    /// User identity token
    pub user_identity_token: IdentityToken,
    /// Preferred language locales
    pub preferred_locales: Vec<String>,
}

impl From<EndpointDescription> for SessionInfo {
    fn from(value: EndpointDescription) -> Self {
        Self {
            endpoint: value,
            user_identity_token: IdentityToken::Anonymous,
            preferred_locales: Vec::new(),
        }
    }
}

impl From<(EndpointDescription, IdentityToken)> for SessionInfo {
    fn from(value: (EndpointDescription, IdentityToken)) -> Self {
        Self {
            endpoint: value.0,
            user_identity_token: value.1,
            preferred_locales: Vec::new(),
        }
    }
}

pub use client::Client;
pub use connect::SessionConnectMode;
pub use event_loop::{SessionActivity, SessionEventLoop, SessionPollResult};
pub use implementation::Session;
use log::{error, info};
pub use request_builder::UARequest;
pub use retry::{DefaultRetryPolicy, RequestRetryPolicy};
pub use services::attributes::{
    HistoryRead, HistoryReadAction, HistoryUpdate, HistoryUpdateAction, Read, Write,
};
pub use services::method::Call;
pub use services::node_management::{AddNodes, AddReferences, DeleteNodes, DeleteReferences};
pub use services::session::{ActivateSession, Cancel, CloseSession, CreateSession};
pub use services::subscriptions::{
    CreateMonitoredItems, CreateSubscription, DataChangeCallback, DeleteMonitoredItems,
    DeleteSubscriptions, EventCallback, ModifyMonitoredItems, ModifySubscription, MonitoredItem,
    OnSubscriptionNotification, SetMonitoringMode, SetPublishingMode, SetTriggering, Subscription,
    SubscriptionActivity, SubscriptionCallbacks, TransferSubscriptions,
};
pub use services::view::{
    Browse, BrowseNext, RegisterNodes, TranslateBrowsePaths, UnregisterNodes,
};

#[allow(unused)]
macro_rules! session_warn {
    ($session: expr, $($arg:tt)*) =>  {
        log::warn!("session:{} {}", $session.session_id(), format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use session_warn;

#[allow(unused)]
macro_rules! session_error {
    ($session: expr, $($arg:tt)*) =>  {
        log::error!("session:{} {}", $session.session_id(), format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use session_error;

#[allow(unused)]
macro_rules! session_debug {
    ($session: expr, $($arg:tt)*) =>  {
        log::debug!("session:{} {}", $session.session_id(), format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use session_debug;

#[allow(unused)]
macro_rules! session_trace {
    ($session: expr, $($arg:tt)*) =>  {
        log::trace!("session:{} {}", $session.session_id(), format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use session_trace;

use opcua_core::ResponseMessage;
use opcua_types::{EndpointDescription, ResponseHeader, StatusCode};

use super::IdentityToken;

/// Process the service result, i.e. where the request "succeeded" but the response
/// contains a failure status code.
pub(crate) fn process_service_result(response_header: &ResponseHeader) -> Result<(), StatusCode> {
    if response_header.service_result.is_bad() {
        info!(
            "Received a bad service result {} from the request",
            response_header.service_result
        );
        Err(response_header.service_result)
    } else {
        Ok(())
    }
}

pub(crate) fn process_unexpected_response(response: ResponseMessage) -> StatusCode {
    match response {
        ResponseMessage::ServiceFault(service_fault) => {
            error!(
                "Received a service fault of {} for the request",
                service_fault.response_header.service_result
            );
            service_fault.response_header.service_result
        }
        _ => {
            error!("Received an unexpected response to the request");
            StatusCode::BadUnknownResponse
        }
    }
}
