use serde::Deserialize;

#[derive(Deserialize)]
pub struct CountViewerRequest {
    pub channel: String,
}