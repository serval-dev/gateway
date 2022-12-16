use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct WebsocketMessage {
    pub op_code: u8,
    pub data: Value
}

impl WebsocketMessage {
    pub fn into_data<T: for<'de> Deserialize<'de>>(&self) -> T {
        serde_json::from_value(self.data).unwrap()
    }
}