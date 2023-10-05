use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use chess_network_protocol::*;

impl GameProtocol {
    pub fn connect_to_server(&mut self) {
        let stream = TcpStream::connect("127.0.0.1:8384");
        self.server = Some(stream);
    }
    pub fn shake_hand_as_client(&mut self) {
        let mut received_hs = serde_json::Deserializer::from_reader(&self.
            server.expect("Should have a server if trying to communicate."));

        let color_of_client = ServerToClientHandshake::deserialize(&mut received_hs).unwrap();

        // shake hand that server has opposite color of client
        let sent_hs = ClientToServerHandshake {
            server_color: match color_of_client {
                chess_network_protocol::Color::White => chess_network_protocol::Color::Black, 
                chess_network_protocol::Color::Black => chess_network_protocol::Color::White, 
            }
        };

        let the_server = &self.server.expect("Should be connected to server when shaking hand.")
        serde_json::to_writer(the_server, &sent_hs).unwrap();

    }
    pub fn communicate_game_as_client(&self) {

    }
}