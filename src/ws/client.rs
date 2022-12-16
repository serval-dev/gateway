use uuid::Uuid;
use warp::ws::WebSocket;

pub struct WebsocketConnection {
    pub id: String,
    pub connected: bool,
    pub disconnnect_time: Option<i64>,
}

impl WebsocketConnection {
    pub fn new() -> Self {
        WebsocketConnection { 
            id: Uuid::new_v4().to_string(),
            connected: false,
            disconnnect_time: None
        }
    }
}