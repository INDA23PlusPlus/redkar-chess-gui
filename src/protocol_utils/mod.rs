use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use chess_network_protocol::*;

pub mod convert;

#[derive(Debug, Copy, Clone)]
pub enum Phase {
    Initialization, 
    Handshake, 
    InPlay,
    Decision,
}

pub struct GameProtocol {
    game: chess_lib::ChessBoard, 
    listener: TcpListener, 
    phase: Phase,
    server: Option<TcpStream>,
    server_found: bool,
    client: Option<TcpStream>,
    client_found: bool,
}

impl GameProtocol {

    // SERVER STARTS HERE

    pub fn new_game(game: chess_lib::ChessBoard) -> Self {
        let listener = TcpListener::bind("127.0.0.1:8384").unwrap();
        Self {
            game: game, 
            listener: listener, 
            client: None,
            client_found: false,
            server: None, 
            server_found: false, 
            phase: Phase::Initialization, 
        }
    }

    pub fn connect_to_client(&mut self) {
        let potential = self.listener.accept();
        match potential {
            Ok((stream, _addr)) => {
                self.client_found = true;
                self.client = Some(stream);
                println!("Connected to a client");
            }
            _ => {}
        }
    } 

    pub fn shake_hand_as_server(&mut self) {
        let c = self.client.as_ref().expect("Should have a stream here!");
        let mut received_hs = serde_json::Deserializer::from_reader(c);
        let deserialized = ClientToServerHandshake::deserialize(&mut received_hs).unwrap();

        let sent_hs = ServerToClientHandshake {
            board: GameProtocol::somasz_board_to_protocol_board(self.game.board),
            moves: vec![],
            features: vec![],
            joever: Joever::Ongoing,
        };

        serde_json::to_writer(c, &sent_hs).expect("failed to send handshake!");
        self.phase = Phase::InPlay;
        // parse deserialized verysion of tcp stream to 
        // need to send ServerToClient Handshake
    }
    
    pub fn communicate_state_as_server(&self) {
        if let Some(stream) = &self.client {
            let state = ServerToClient::State {
                board: Self::somasz_board_to_protocol_board(self.game.board),
                moves: vec![],
                joever: match self.phase {
                    Phase::InPlay => Joever::Ongoing, 
                    Phase::Decision => todo!(),
                    _ => Joever::Ongoing,
                    // little sus to set this as ongoing
                },
                // putting in random move for now
                move_made: chess_network_protocol::Move {
                    start_x: 0,
                    start_y: 0,
                    end_x: 0,
                    end_y: 0,
                    promotion: Piece::None,
                },
            };
            serde_json::to_writer(stream, &state).expect("Not able to send game state as server!");
        }
    }    


    // CLIENT STARTS HERE

    pub fn connect_to_server(&mut self) {
        let stream = TcpStream::connect("127.0.0.1:8384").expect("Couldn't connect to server!");
        self.server = Some(stream);
    }
    pub fn shake_hand_as_client(&mut self, chosen_color: chess_network_protocol::Color) {
        let mut received_hs = serde_json::Deserializer::from_reader(&self.
            server.as_ref().expect("Should have a server if trying to communicate!"));

        // shake hand that server has opposite color of client
        let sent_hs = ClientToServerHandshake {
            server_color: chosen_color,
        };

        let the_server = self.server.as_ref().expect("Should be connected to server when shaking hand!");
        serde_json::to_writer(the_server, &sent_hs).unwrap();

    }

    pub fn communicate_move_as_client(&self) {
        if let Some(stream) = &self.server {
            let some_move = chess_network_protocol::Move {
                start_x: 0,
                start_y: 0,
                end_x: 0,
                end_y: 0,
                promotion: Piece::None,
            };
            serde_json::to_writer(stream, &some_move).expect("Not able to send move as client!");
        }
    }    

    pub fn resign_as_client(&self) {
        if let Some(stream) = &self.server {
            let resignation = ClientToServer::Resign;
            serde_json::to_writer(stream, &resignation).expect("Not able to resign as client!");
        }
    }

    pub fn draw_as_client(&self) {
        if let Some(stream) = &self.server {
            let the_draw = ClientToServer::Draw;
            serde_json::to_writer(stream, &the_draw).expect("Not able to signal draw as client!");
        }
    }

    // todo! communicate_decision / Error
    // todo! maybe make a possible_moves generator
    // todo! do_move
    // todo! conversions
}