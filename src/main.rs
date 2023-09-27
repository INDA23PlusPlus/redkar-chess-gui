use ggez::event::MouseButton;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Rect};
use ggez::GameError;
use ggez::glam::*;
use std::{env, path};
use chess_lib::*;

const SQUARE: f32 = 125.0;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Col {
    White, 
    Black 
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Pawn, 
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub piece: PieceType, 
    pub color: Col, 
}


struct State {
    chessboard: [[Option<Piece>; 8]; 8],
    mouse_x: f32,
    mouse_y: f32,
    cur_square_x: f32,
    cur_square_y: f32,
    square_selected: bool, 
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

                if self.chessboard[y][x].is_some() {
                    let print_pos = Vec2::new(x as f32 * SQUARE, y as f32 * SQUARE);
                    match self.chessboard[y][x] {
                        Some(p) => {
                            match p  {
                                Piece{piece: PieceType::Pawn, color: Col::White} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/w_pawn.png").unwrap()), print_pos);
                                },
                                Piece{piece: PieceType::Pawn, color: Col::Black} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/b_pawn.png").unwrap()), print_pos);
                                },
                                Piece{piece: PieceType::Knight, color: Col::White} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/w_knight.png").unwrap()), print_pos);    
                                },
                                Piece{piece: PieceType::Knight, color: Col::Black} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/b_knight.png").unwrap()), print_pos);
                                },
                                Piece{piece: PieceType::Bishop, color: Col::White} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/w_bishop.png").unwrap()), print_pos);
                                },
                                Piece{piece: PieceType::Bishop, color: Col::Black} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/b_bishop.png").unwrap()), print_pos); 
                                },
                                Piece{piece: PieceType::Rook, color: Col::White} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/w_rook.png").unwrap()), print_pos); 
                                },
                                Piece{piece: PieceType::Rook, color: Col::Black} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/b_rook.png").unwrap()), print_pos); 
                                },
                                Piece{piece: PieceType::Queen, color: Col::White} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/w_queen.png").unwrap()), print_pos); 
                                },
                                Piece{piece: PieceType::Queen, color: Col::Black} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/b_queen.png").unwrap()), print_pos);    
                                },
                                Piece{piece: PieceType::King, color: Col::White} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/w_king.png").unwrap()), print_pos);  
                                },
                                Piece{piece: PieceType::King, color: Col::Black} => {
                                    canvas.draw(&(graphics::Image::from_path(ctx, "/b_king.png").unwrap()), print_pos);
                                },
                            }
                        }, 
                        None => { }
                    }
                }
            }
            // here we need to refer to our own board and draw the pieces accordingly
        }
        // graphics::Image::from_path(ctx, "/w-pawn.png")?;

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
        self.cur_square_x = x_pos;
        self.cur_square_y = y_pos;
        // check if the current position 
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
    let state = State {
        chessboard:  {
            [[
                Some(Piece{piece: PieceType::Rook, color: Col::White}),  
                Some(Piece{piece: PieceType::Knight, color: Col::White}), 
                Some(Piece{piece: PieceType::Bishop, color: Col::White}), 
                Some(Piece{piece: PieceType::King, color: Col::White}), 
                Some(Piece{piece: PieceType::Queen, color: Col::White}), 
                Some(Piece{ piece: PieceType::Bishop, color: Col::White}), 
                Some(Piece{piece: PieceType::Knight, color: Col::White}), 
                Some(Piece{piece: PieceType::Rook, color: Col::White}) 
            ],
            [Some(Piece{piece: PieceType::Pawn, color: Col::White}); 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [Some(Piece{piece: PieceType::Pawn, color: Col::Black}); 8],
            [
                Some(Piece{piece: PieceType::Rook, color: Col::Black}),  
                Some(Piece{piece: PieceType::Knight, color: Col::Black}), 
                Some(Piece{piece: PieceType::Bishop, color: Col::Black}), 
                Some(Piece{piece: PieceType::King, color: Col::Black}), 
                Some(Piece{piece: PieceType::Queen, color: Col::Black}), 
                Some(Piece{piece: PieceType::Bishop, color: Col::Black}), 
                Some(Piece{piece: PieceType::Knight, color: Col::Black}), 
                Some(Piece{piece: PieceType::Rook, color: Col::Black}) 
            ]]
        }, 
        mouse_x: 0.0, mouse_y: 0.0, 
        cur_square_x: 0.0, cur_square_y: 0.0,
        square_selected: false,
    };

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Chess", "Raunak Redkar")
    .add_resource_path(resource_dir)
    .window_mode(ggez::conf::WindowMode::default().dimensions(SQUARE * 8.0, SQUARE * 8.0))
    .build()
    .unwrap();
    ggez::event::run(ctx, event_loop, state);
}
