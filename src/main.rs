use ggez::event::MouseButton;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Rect, Image};
use ggez::GameError;
use ggez::glam::*;
use std::{env, path};
use chess_lib;

const SQUARE: f32 = 125.0;

struct State {
    w_pawn: graphics::Image,
    b_pawn: graphics::Image,
    w_knight: graphics::Image,
    b_knight: graphics::Image,
    w_bishop: graphics::Image,
    b_bishop: graphics::Image,
    w_rook: graphics::Image,
    b_rook: graphics::Image,
    w_queen: graphics::Image,
    b_queen: graphics::Image,
    w_king: graphics::Image,
    b_king: graphics::Image,
    game: chess_lib::ChessBoard,
    mouse_x: f32,
    mouse_y: f32,
    cur_square_x: f32,
    cur_square_y: f32,
    square_selected: bool, 
    piece_selected: bool, 
    piece_x: usize,
    piece_y: usize,
    possible_moves: Vec<(usize, usize)>,
    // we need to make a board ourselves starting at the starting position
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        let w_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, SQUARE, SQUARE),
            Color::WHITE,
        )?;
        let b_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, SQUARE, SQUARE),
            Color::from_rgb(106, 169, 210),
        )?;
        let selected_square = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, SQUARE, SQUARE),
            // Color::MAGENTA,
            Color::from_rgb(236, 177, 41),
        )?;

        for y in 0..8 {
            for x in 0..8 {
                if self.square_selected && (self.cur_square_x / SQUARE).floor() as usize == x && (self.cur_square_y / SQUARE).floor() as usize == y {
                    canvas.draw(&selected_square, 
                        Vec2::new(
                            (self.cur_square_x / SQUARE).floor() * SQUARE, 
                            (self.cur_square_y / SQUARE).floor() * SQUARE
                        )
                    );
                }
                else if (x + y) % 2 == 0 {
                    canvas.draw(&w_square, Vec2::new(x as f32 * SQUARE, y as f32 * SQUARE));
                }
                else {
                    canvas.draw(&b_square, Vec2::new(x as f32 * SQUARE, y as f32 * SQUARE));
                }

                if self.game.board[y][x].is_some() {
                    let print_pos = Vec2::new(x as f32 * SQUARE, y as f32 * SQUARE);
                    match &self.game.board[y][x] {
                        Some(p) => {
                            match p  {
                                chess_lib::ChessPiece::Pawn(chess_lib::Color::White) => {
                                    canvas.draw(&self.w_pawn, print_pos);
                                },
                                chess_lib::ChessPiece::Pawn(chess_lib::Color::Black) => {
                                    canvas.draw(&self.b_pawn, print_pos);
                                },
                                chess_lib::ChessPiece::Knight(chess_lib::Color::White) => {
                                    canvas.draw(&self.w_knight, print_pos);
                                },
                                chess_lib::ChessPiece::Knight(chess_lib::Color::Black) => {
                                    canvas.draw(&self.b_knight, print_pos);
                                },
                                chess_lib::ChessPiece::Bishop(chess_lib::Color::White) => {
                                    canvas.draw(&self.w_bishop, print_pos);
                                },
                                chess_lib::ChessPiece::Bishop(chess_lib::Color::Black) => {
                                    canvas.draw(&self.b_bishop, print_pos);
                                },
                                chess_lib::ChessPiece::Rook(chess_lib::Color::White) => {
                                    canvas.draw(&self.w_rook, print_pos);
                                },
                                chess_lib::ChessPiece::Rook(chess_lib::Color::Black) => {
                                    canvas.draw(&self.b_rook, print_pos);
                                },
                                chess_lib::ChessPiece::Queen(chess_lib::Color::White) => {
                                    canvas.draw(&self.w_queen, print_pos);
                                },
                                chess_lib::ChessPiece::Queen(chess_lib::Color::Black) => {
                                    canvas.draw(&self.b_queen, print_pos);
                                },
                                chess_lib::ChessPiece::King(chess_lib::Color::White) => {
                                    canvas.draw(&self.w_king, print_pos);
                                },
                                chess_lib::ChessPiece::King(chess_lib::Color::Black) => {
                                    canvas.draw(&self.b_king, print_pos);
                                },
                            }
                        }, 
                        None => { }
                    }
                }
            }
        }

        canvas.finish(ctx)?;

        Ok(())
    }
    /* 
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x_pos: f32, y_pos: f32, _dx: f32, _dy: f32) -> GameResult {
        self.mouse_x = x_pos;
        self.mouse_y = y_pos;
        Ok(())
    }
    */

    fn mouse_button_down_event(&mut self,_ctx: &mut Context, button: MouseButton, x_pos: f32, y_pos: f32) -> GameResult {
        // if it wasnt left click ignore?    
        if self.square_selected {
            self.square_selected = false;
        }
        self.cur_square_x = x_pos;
        self.cur_square_y = y_pos;
        let cur_x = (self.cur_square_x / SQUARE).floor() as usize; 
        let cur_y = (self.cur_square_y / SQUARE).floor() as usize;
        if self.possible_moves.contains(&(cur_x, cur_y)) {
            self.game.set_piece((self.piece_x, self.piece_y), (cur_x, cur_y));
            self.game.increase_turn();
        }
        self.piece_selected = false;
        Ok(())
    }
    
    fn mouse_button_up_event(&mut self,_ctx: &mut Context, button: MouseButton, x_pos: f32, y_pos: f32) -> GameResult {
        // if it wasnt left click ignore?    
        if self.square_selected { 
            self.square_selected = false;
        }
        else {
            self.square_selected = true;
        }
        if self.piece_selected {
            self.piece_selected = false;
        }
        else {
            let cur_x = (self.cur_square_x / SQUARE).floor() as usize; 
            let cur_y = (self.cur_square_y / SQUARE).floor() as usize;
            match self.game.select_piece((cur_x, cur_y), &self.game.faction_decider()) {
                Some(x) => {
                    self.piece_x = (self.cur_square_x / SQUARE).floor() as usize;
                    self.piece_y = (self.cur_square_y / SQUARE).floor() as usize;
                    self.piece_selected = true;                                    
                    self.possible_moves = x;
                }
                None => {}
            }
        }
        Ok(())
    }

}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Chess", "Raunak Redkar")
    .add_resource_path(resource_dir)
    .window_mode(ggez::conf::WindowMode::default().dimensions(SQUARE * 8.0, SQUARE * 8.0))
    .build()
    .unwrap();
    let state = State {
        game: chess_lib::ChessBoard::create(), 
        mouse_x: 0.0, mouse_y: 0.0, 
        cur_square_x: 0.0, cur_square_y: 0.0,
        piece_x: 0, piece_y: 0, 
        piece_selected: false,
        square_selected: false,
        possible_moves: Vec::<(usize, usize)>::new(),
        w_pawn: graphics::Image::from_path(&ctx, "/w_pawn.png").unwrap(),
        b_pawn: graphics::Image::from_path(&ctx, "/b_pawn.png").unwrap(),
        w_knight: graphics::Image::from_path(&ctx, "/w_knight.png").unwrap(),
        b_knight: graphics::Image::from_path(&ctx, "/b_knight.png").unwrap(),
        w_bishop: graphics::Image::from_path(&ctx, "/w_bishop.png").unwrap(),
        b_bishop: graphics::Image::from_path(&ctx, "/b_bishop.png").unwrap(),
        w_rook: graphics::Image::from_path(&ctx, "/w_rook.png").unwrap(),
        b_rook: graphics::Image::from_path(&ctx, "/b_rook.png").unwrap(),
        w_queen: graphics::Image::from_path(&ctx, "/w_queen.png").unwrap(),
        b_queen: graphics::Image::from_path(&ctx, "/b_queen.png").unwrap(),
        w_king: graphics::Image::from_path(&ctx, "/w_king.png").unwrap(),
        b_king: graphics::Image::from_path(&ctx, "/b_king.png").unwrap(),
    };
    ggez::event::run(ctx, event_loop, state);
}
