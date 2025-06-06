use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct KafkaPayload {
    user_id: String,
    chanel: String,
    status: String,
}

impl KafkaPayload {
    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn set_user_id(&mut self, user_id: String) -> &mut Self {
        self.user_id = user_id;
        self
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: String) -> &mut Self {
        self.status = status;
        self
    }

     pub fn get_chanel(&self) -> String {
        self.chanel.clone()
    }

    pub fn set_chanel(&mut self, chanel: String) -> &mut Self {
        self.chanel = chanel;
        self
    }
}
