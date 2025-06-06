use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::schema::viewer::Viewer;

#[derive(Serialize, Debug, Clone)]
pub struct ViewerResponse {
    user_id: String,
    channel: String,
    create_at: DateTime<Utc>,
}

impl From<Viewer> for ViewerResponse {
    fn from(viewer: Viewer) -> Self {
        ViewerResponse {
            user_id: viewer.get_user_id(),
            channel: viewer.get_channel(),
            create_at: viewer.get_create_at(),
        }
    }
}
