//! Gameboard controller.

use piston::input::GenericEvent;

use Gameboard;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
    /// Selected Cell.
    pub selected_cell: Option<[usize; 2]>,
    pub cursor_pos: [f64; 2],
    pub legal_moves: Vec<[usize; 2]>,
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            selected_cell:  None,
            cursor_pos: [1.0; 2],
            legal_moves: Vec::new(),
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size:f64, e: &E) {
        use piston::input::{Button, MouseButton};

        if let Some(pos) = e.mouse_cursor_args(){
            self.cursor_pos = pos;
        }



        if let Some(Button::Mouse(MouseButton::Left))=e.press_args(){

            let x = self.cursor_pos[0]-pos[0];
            let y = self.cursor_pos[1]-pos[1];
            if x >= 0.0 && x<size && y>=0.0 && y<size {
                let cell_x = (x / size*8.0) as usize;
                let cell_y = (y / size*8.0) as usize;

                if self.legal_moves.len()>0{
                    for num in &self.legal_moves {
                        if cell_x==num[0] && cell_y==num[1]{
                            if let Some (A) = self.selected_cell{
                                self.gameboard.move_piece(A,[cell_x,cell_y]);
                                println!("Legal Move!");
                            }

                        }
                    }
                    self.legal_moves = Vec::new();
                }
                self.selected_cell = Some ([cell_x, cell_y] );
                let selected_value = self.gameboard.get_cell_value([cell_x, cell_y]);
                let tile_ownership = self.gameboard.check_tile_ownership([cell_x, cell_y]);
                let legal_move_set = self.gameboard.generate_legal_move_set([cell_x, cell_y]);
                self.legal_moves = legal_move_set;
                println!("{} , {:?}, {:?}", selected_value, self.selected_cell, self.legal_moves);
            }
        }

    }
}