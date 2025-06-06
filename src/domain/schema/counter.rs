use scylla::{value::Counter, DeserializeRow, DeserializeValue};

#[derive(Debug, Clone, PartialEq, Eq, DeserializeRow, DeserializeValue)]
pub struct CounterViewer {
    channel: String,
    viewers: Counter,
}

impl CounterViewer {
    pub fn new (channel: String, viewers: Counter) -> CounterViewer {
        CounterViewer { channel: channel, viewers: viewers }
    }

    pub fn get_channel(&self) -> String {
        self.channel.clone()
    }

    pub fn set_channel(&mut self, channel: String) -> &mut Self {
        self.channel = channel;
        self
    }

    pub fn get_viewers(&self) -> Counter {
        self.viewers
    }

    pub fn set_viewers(&mut self, count: Counter) -> &mut Self {
        self.viewers = count;
        self
    }
}