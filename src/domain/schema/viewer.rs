use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Viewer {
    user_id: String,
    channel: String,
    create_at: DateTime<Utc>
}

impl Viewer {
    pub fn new(user_id: String, channel: String, create_at: DateTime<Utc>) -> Viewer {
        Viewer { user_id, channel, create_at }
    }

    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn set_user_id(&mut self, user_id: String) -> &mut Self {
        self.user_id = user_id;
        self
    }

    pub fn get_channel(&self) -> String {
        self.channel.clone()
    }

    pub fn set_channel(&mut self, channel: String) -> &mut Self {
        self.channel = channel;
        self
    }

    pub fn get_create_at(&self) -> DateTime<Utc> {
        self.create_at
    }

    pub fn set_create_at(&mut self, create_at: DateTime<Utc>) -> &mut Self {
        self.create_at = create_at;
        self
    }
}
