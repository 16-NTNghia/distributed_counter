use serde::Serialize;

use crate::domain::schema::{counter::CounterViewer};

#[derive(Serialize, Debug, Clone)]
pub struct CounterViewerResponse {
    channel: String,
    viewers: i64,
}

impl From<CounterViewer> for CounterViewerResponse {
    fn from(counter_viewer: CounterViewer) -> Self {
        CounterViewerResponse {
            channel: counter_viewer.get_channel(),
            viewers: counter_viewer.get_viewers().0,
        }
    }
}
