use serde::{Deserialize, Serialize};

use crate::domain::schema::viewer::Viewer;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ViewerCreateRequest {
    user_id: String,
    channel: String,
    status: String,
}

impl ViewerCreateRequest {
    pub fn new(user_id: String, channel: String, status: String) -> ViewerCreateRequest {
        ViewerCreateRequest {
            user_id,
            channel,
            status,
        }
    }

    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn set_user_id(&mut self, user_id: String) -> &mut Self {
        self.user_id = user_id;
        self
    }

    pub fn get_chanel(&self) -> String {
        self.channel.clone()
    }

    pub fn set_chanel(&mut self, channel: String) -> &mut Self {
        self.channel = channel;
        self
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: String) -> &mut Self {
        self.status = status;
        self
    }
}

impl From<ViewerCreateRequest> for Viewer {
    fn from(req: ViewerCreateRequest) -> Self {
        Viewer::new (
            req.user_id.to_string(),
            req.channel.to_string(),
            chrono::Utc::now(),
        )
    }
}
