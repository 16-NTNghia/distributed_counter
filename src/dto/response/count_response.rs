use serde::Serialize;

#[derive(Serialize, Debug, Clone, Default)]
pub struct CountResponse {
    count: i64,
}

impl From<i64> for CountResponse {
    fn from(count: i64) -> Self {
        CountResponse { count }
    }
}
