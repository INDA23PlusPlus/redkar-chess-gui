use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use chess_network_protocol::*;

#[derive(Debug, Copy, Clone)]
pub enum Phase {
    Handshake, 
    InPlay,
    Decision,
}

pub struct GameServer {
    game: chess_lib::ChessBoard, 
    listener: TcpListener, 
    phase: Option<Phase>,
    client: Option<TcpStream>,
    client_found: bool,
}

impl GameServer {
    pub fn new_game(game: chess_lib::ChessBoard) -> Self {
        let listener = TcpListener::bind("127.0.0.1:8384").unwrap();
        Self {
            game: game, 
            listener: listener, 
            client_found: false,
            client: None,
            phase: None, 
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

    pub fn shake_hand(&mut self) {
        let c = self.client.as_ref().unwrap();
        let mut hs = serde_json::Deserializer::from_reader(c);
        let deserialized = ClientToServerHandshake::deserialize(&mut hs);
        // parse deserialized verysion of tcp stream to 
        self.phase = Some(Phase::Handshake);
        // need to send ServerToClient Handshake
    }
    
    pub fn communicate_game(&self) {
        if let Some(stream) = &self.client {
            let state = ServerToClient::State {
                board: Self::somasz_board_to_protocol_board(self.game.board),
                moves: vec![],
                joever: match self.phase {
                    Some(x) => match x {
                        Phase::Decision => todo!(),
                        _ => Joever::Ongoing,
                    },
                    // little sus to set this as ongoing
                    None => Joever::Ongoing,
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
            serde_json::to_writer(stream, &state).expect("Not able to send");
        }
        // get a board: [chess_network_protocol::Piece; 8]; 8] by converting from chess_lib::Chessboard.board
        // try to keep track of moves yourself
        // if there is a stream running, make a ServerToClient::State 
    }    
    // todo! communicate_decision / Error
    // todo! maybe make a possible_moves generator
    // todo! do_move
    // todo! conversions
}