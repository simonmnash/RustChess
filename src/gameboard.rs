//! Game board logic

const SIZE: usize=8;

///Stores game board information.
pub struct Gameboard {
    ///Stores content of the cells.
    ///0 is the empty cell.
    pub cells:[[u8; SIZE]; SIZE],
    pub perspective: bool,
    pub turn_number: u32,
}

impl Gameboard {
    ///Creates a new game board.
    pub fn new() ->Gameboard {
        Gameboard {
            perspective: false,
            cells: [[7, 3, 5, 9, 11, 2, 3, 7],
                    [1, 1, 1, 1, 1, 1, 2, 1],
                    [0, 0, 0, 0, 0, 0, 0, 2],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [2, 2, 2, 2, 2, 2, 2, 2],
                    [8, 4, 6, 10, 12, 6, 4, 8]],
            turn_number: 0,
        }
    }
    /// Gets the character at cell location.
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]] {
            1 => 'P',
            2 => 'p',
            3 => 'N',
            4 => 'n',
            5 => 'B',
            6 => 'b',
            7 => 'R',
            8 => 'r',
            9 => 'Q',
            10 => 'q',
            11 => 'K',
            12 => 'k',
            _ => return None,
        })
    }
    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        self.cells[ind[1]][ind[0]] = val;
    }

    pub fn get_cell_value(&mut self, ind: [usize; 2]) -> u8 {
        self.cells[ind[1]][ind[0]]
    }
    pub fn check_tile_ownership(&mut self, ind: [usize; 2]) -> bool{
        let value=self.cells[ind[1]][ind[0]];
        if value==0 {
            false
        }
        else if value % 2 ==0 {
            true
        }
        else {
            false
        }
    }

    pub fn check_against_board_edge(&mut self, x: i32, y: i32) -> bool {
        if x>7 || y>7 || x<0 || y<0 {
            false
        }
        else {
            true
        }
    }

    pub fn generate_legal_move_set(&mut self, ind: [usize; 2]) -> Vec<[usize; 2]>{
        let mut vec: Vec<[usize; 2]> = Vec::new();
        let ownership=self.check_tile_ownership(ind);
        if ownership {
            let cell_value = self.get_cell_value(ind);
            //White Pawn Legal Move Generation
            if cell_value == 2 {
                if self.check_against_board_edge(ind[0] as i32, (ind[1] as i32) - 1) {
                    let one_forward = [ind[0], ind[1] - 1];
                    vec.push(one_forward);
                }


                if ind[1] == 6 {
                    if self.check_against_board_edge(ind[0] as i32, (ind[1] as i32) - 2) {
                        let two_forward = [ind[0], ind[1] - 2];
                        vec.push(two_forward);
                    }
                }

            }
                else if cell_value == 4 || cell_value ==3 {

                    if self.check_against_board_edge((ind[0] as i32) - 1, (ind[1] as i32) - 2) {
                        vec.push([ind[0] - 1, ind[1] - 2]);
                    }
                    if self.check_against_board_edge((ind[0] as i32) + 1, (ind[1] as i32) - 2) {
                       vec.push([ind[0] + 1, ind[1] - 2]);
                    }
                    if self.check_against_board_edge((ind[0] as i32) + 1, (ind[1] as i32) + 2) {
                        vec.push([ind[0] + 1, ind[1] + 2]);
                    }
                    if self.check_against_board_edge((ind[0] as i32) - 1, (ind[1] as i32) + 2) {
                        vec.push([ind[0] - 1, ind[1] + 2]);
                    }
                    if self.check_against_board_edge((ind[0] as i32) - 2, (ind[1] as i32) - 1) {
                        vec.push([ind[0] - 2, ind[1] - 1]);
                    }
                    if self.check_against_board_edge((ind[0] as i32) + 2, (ind[1] as i32) - 1) {
                        vec.push([ind[0] + 2, ind[1] - 1]);
                    }
                    if self.check_against_board_edge((ind[0] as i32) + 2, (ind[1] as i32) + 1) {
                        vec.push([ind[0] + 2, ind[1] + 1]);
                    }
                    if self.check_against_board_edge((ind[0] as i32) - 2, (ind[1] as i32) + 1) {
                        vec.push([ind[0] - 2, ind[1] + 1]);
                    }
                }
            else if cell_value == 11 || cell_value == 12 {
                if self.check_against_board_edge((ind[0] as i32) + 1, (ind[1] as i32) + 1) {
                    vec.push([ind[0] + 1, ind[1] + 1]);
                }
                if self.check_against_board_edge((ind[0] as i32) + 1, (ind[1] as i32) + 0) {
                    vec.push([ind[0] + 1, ind[1] + 0]);
                }
                if self.check_against_board_edge((ind[0] as i32) + 1, (ind[1] as i32) - 1) {
                    vec.push([ind[0] + 1, ind[1] - 1]);
                }
                if self.check_against_board_edge((ind[0] as i32) + 0, (ind[1] as i32) + 1) {
                    vec.push([ind[0] + 0, ind[1] + 1]);
                }
                if self.check_against_board_edge((ind[0] as i32) + 0, (ind[1] as i32) - 1) {
                    vec.push([ind[0] + 0, ind[1] - 1]);
                }
                if self.check_against_board_edge((ind[0] as i32) -1, (ind[1] as i32) +1) {
                    vec.push([ind[0] - 1, ind[1] + 1]);
                }
                if self.check_against_board_edge((ind[0] as i32) -1, (ind[1] as i32) + 0) {
                    vec.push([ind[0] - 1, ind[1] + 0]);
                }
                if self.check_against_board_edge((ind[0] as i32) - 1, (ind[1] as i32) - 1) {
                    vec.push([ind[0] - 1, ind[1] - 1]);
                }
                }
            }

        vec
    }

    pub fn move_piece(&mut self, origin: [usize; 2], target: [usize; 2]) {
        let piece=self.get_cell_value(origin);
        self.set(target, piece);
        self.set(origin, 0);

    }

}
