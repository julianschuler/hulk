mod api_id;
mod mode;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::Error, rpc::api_id::ApiId};

pub use crate::rpc::mode::Mode;

#[repr(C)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    uuid: String,
    header: String,
    body: String,
}

impl Request {
    fn new(api_id: ApiId, body: impl Into<String>) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let header = serde_json::to_string(&RequestHeader { api_id })
            .expect("JSON serialization should never fail");
        let body = body.into();

        Self { uuid, header, body }
    }

    pub fn change_mode(mode: Mode) -> Self {
        let body = serde_json::to_string(&ModeBody { mode })
            .expect("JSON serialization should never fail");

        Self::new(ApiId::ChangeMode, body)
    }

    pub fn get_mode() -> Self {
        Self::new(ApiId::GetMode, "")
    }

    pub fn enter_wbc_gait() -> Self {
        Self::new(ApiId::EnterWBCGait, "")
    }

    pub fn exit_wbc_gait() -> Self {
        Self::new(ApiId::ExitWBCGait, "")
    }

    pub fn get_up() -> Self {
        Self::new(ApiId::GetUp, "")
    }
}

#[repr(C)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    uuid: String,
    header: String,
    body: String,
}

impl Response {
    pub fn is_response_to(&self, request: &Request) -> bool {
        self.uuid == request.uuid
    }

    pub fn status(&self) -> Result<usize, Error> {
        let header: ResponseHeader = serde_json::from_str(&self.header)?;

        Ok(header.status)
    }

    pub fn mode(&self) -> Result<Mode, Error> {
        let body: ModeBody = serde_json::from_str(&self.body)?;

        Ok(body.mode)
    }
}

#[derive(Serialize)]
struct RequestHeader {
    api_id: ApiId,
}

#[derive(Deserialize)]
struct ResponseHeader {
    status: usize,
}

#[derive(Serialize, Deserialize)]
struct ModeBody {
    mode: Mode,
}
