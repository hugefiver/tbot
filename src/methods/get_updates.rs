use super::send_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{parameters::UpdateKind, Update},
};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[must_use]
pub(crate) struct GetUpdates<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<&'a [UpdateKind]>,
}

impl<'a> GetUpdates<'a> {
    pub(crate) const fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        offset: Option<isize>,
        limit: Option<u8>,
        timeout: Option<u64>,
        allowed_updates: Option<&'a [UpdateKind]>,
    ) -> Self {
        Self {
            client,
            token,
            offset,
            limit,
            timeout,
            allowed_updates,
        }
    }
}

impl GetUpdates<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<Update>, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "getUpdates",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
