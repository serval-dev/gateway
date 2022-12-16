use std::{collections::HashMap};

use futures::{StreamExt, SinkExt};

use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{
    ws::{Ws, WebSocket, Message}
};

use crate::state::GatewayState;

use self::client::WebsocketConnection;

pub mod client;
pub mod message;
pub struct WebsocketHandler {
    pub connections: HashMap<String, WebsocketConnection>
}

impl WebsocketHandler {
    pub async fn connect(&self, state: GatewayState, socket: WebSocket) {
        let (mut client_sx, mut client_recv) = socket.split();

        // Use an unbounded channel for message buffers.
        let (sx, recv) = mpsc::unbounded_channel::<Result<Message, warp::Error>>();
        let mut recv_stream = UnboundedReceiverStream::new(recv);

        // Attach the sending channel to our buffer.
        tokio::task::spawn(async move {
            while let Some(message) = recv_stream.next().await {
                client_sx
                    .send(message.unwrap())
                    .await;
            }
        });

        // Create a client with the websocket at its forefront, and check if its id already exists.
        let mut base_client = WebsocketConnection::new();
        let mut client = loop {
            let potential_client = self.connections.get(&base_client.id);

            match potential_client {
                Some(client) => {
                    // This client exists, so we'll need to reassign a new id.
                    base_client.id = Uuid::new_v4().to_string()
                },

                None => {
                    // This client has a unique id, so we're fine.
                    break base_client;
                }
            }
            
        };

        while let Some(message) = client_recv.next().await {
            if (message.is_ok()) {
                // Handle the message, and reference the client mutably.
                self.message(&mut client, message.unwrap());
            }
        }
    }

    pub async fn disconnect(&self, client: &mut WebsocketConnection) {

    }

    pub async fn message(&self, client: &mut WebsocketConnection, message: Message) {
    }

    pub fn new() -> Self {
        WebsocketHandler { 
            connections: HashMap::new()
        }
    }
}
