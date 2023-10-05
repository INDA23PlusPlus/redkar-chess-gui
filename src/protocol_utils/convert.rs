use crate::protocol_utils::GameProtocol;

// hope this works across files
impl GameProtocol {
    pub fn somasz_to_protocol_piece(somasz_piece: Option<chess_lib::ChessPiece>) -> chess_network_protocol::Piece {
        match somasz_piece {
            Some(the_piece) => match the_piece {
                chess_lib::ChessPiece::Pawn(chess_lib::Color::White) => chess_network_protocol::Piece::WhitePawn,
                chess_lib::ChessPiece::Pawn(chess_lib::Color::Black) => chess_network_protocol::Piece::BlackPawn,
                chess_lib::ChessPiece::Knight(chess_lib::Color::White) => chess_network_protocol::Piece::WhiteKnight,
                chess_lib::ChessPiece::Knight(chess_lib::Color::Black) => chess_network_protocol::Piece::BlackKnight,
                chess_lib::ChessPiece::Bishop(chess_lib::Color::White) => chess_network_protocol::Piece::WhiteBishop,
                chess_lib::ChessPiece::Bishop(chess_lib::Color::Black) => chess_network_protocol::Piece::BlackBishop,
                chess_lib::ChessPiece::Rook(chess_lib::Color::White) => chess_network_protocol::Piece::WhiteRook,
                chess_lib::ChessPiece::Rook(chess_lib::Color::Black) => chess_network_protocol::Piece::BlackRook,
                chess_lib::ChessPiece::Queen(chess_lib::Color::White) => chess_network_protocol::Piece::WhiteQueen,
                chess_lib::ChessPiece::Queen(chess_lib::Color::Black) => chess_network_protocol::Piece::BlackQueen,
                chess_lib::ChessPiece::King(chess_lib::Color::White) => chess_network_protocol::Piece::WhiteKing,
                chess_lib::ChessPiece::King(chess_lib::Color::Black) => chess_network_protocol::Piece::BlackKing,
            }
            None => chess_network_protocol::Piece::None,
        }
    }

    pub fn protocol_to_somasz_piece(protocol_piece: Option<chess_network_protocol::Piece>) -> Option<chess_lib::ChessPiece> {
        match protocol_piece {
            Some(chess_network_protocol::Piece::WhitePawn) => Some(chess_lib::ChessPiece::Pawn(chess_lib::Color::White)),
            Some(chess_network_protocol::Piece::BlackPawn) => Some(chess_lib::ChessPiece::Pawn(chess_lib::Color::Black)),
            Some(chess_network_protocol::Piece::WhiteKnight) => Some(chess_lib::ChessPiece::Knight(chess_lib::Color::White)),
            Some(chess_network_protocol::Piece::BlackKnight) => Some(chess_lib::ChessPiece::Knight(chess_lib::Color::Black)),
            Some(chess_network_protocol::Piece::WhiteBishop) => Some(chess_lib::ChessPiece::Bishop(chess_lib::Color::White)),
            Some(chess_network_protocol::Piece::BlackBishop) => Some(chess_lib::ChessPiece::Bishop(chess_lib::Color::Black)),
            Some(chess_network_protocol::Piece::WhiteRook) => Some(chess_lib::ChessPiece::Rook(chess_lib::Color::White)),
            Some(chess_network_protocol::Piece::BlackRook) => Some(chess_lib::ChessPiece::Rook(chess_lib::Color::Black)),
            Some(chess_network_protocol::Piece::WhiteQueen) => Some(chess_lib::ChessPiece::Queen(chess_lib::Color::White)),
            Some(chess_network_protocol::Piece::BlackQueen) => Some(chess_lib::ChessPiece::Queen(chess_lib::Color::Black)),
            Some(chess_network_protocol::Piece::WhiteKing) => Some(chess_lib::ChessPiece::King(chess_lib::Color::White)),
            Some(chess_network_protocol::Piece::BlackKing) => Some(chess_lib::ChessPiece::King(chess_lib::Color::Black)),
            Some(chess_network_protocol::Piece::None) => None,
            None => None, 
        }
    }

    pub fn somasz_board_to_protocol_board(mut somasz_board: [[Option<chess_lib::ChessPiece>; 8]; 8]) -> [[chess_network_protocol::Piece; 8]; 8] {
        let mut protocol_board = [[chess_network_protocol::Piece::None; 8]; 8];
        for i in 1..8 {
            for j in 1..8 {
                protocol_board[i as usize][j as usize] = Self::somasz_to_protocol_piece(somasz_board[i as usize][j as usize]);
            }
        }
        return protocol_board;
    }

    pub fn protocol_board_to_somasz_board(mut protocol_board: [[Option<chess_network_protocol::Piece>; 8]; 8]) -> [[Option<chess_lib::ChessPiece>; 8]; 8] {
        let mut somasz_board = [[Option::<chess_lib::ChessPiece>::None; 8]; 8];
        for i in 1..8 {
            for j in 1..8 {
                somasz_board[i as usize][j as usize] = Self::protocol_to_somasz_piece(protocol_board[i as usize][j as usize]);
            }
        }
        return somasz_board;
    }

}