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
            cells: [[7, 3, 5, 9, 11, 5, 3, 7],
                    [1, 1, 1, 1, 1, 1, 1, 1],
                    [0, 0, 0, 0, 0, 0, 0, 0],
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
    pub fn generate_legal_move_set(&mut self, ind: [usize; 2]) -> Vec<[usize; 2]>{
        let mut vec: Vec<[usize; 2]> = Vec::new();
        let ownership=self.check_tile_ownership(ind);
        if ownership {
            let cell_value = self.get_cell_value(ind);
            //White Pawn Legal Move Generation
            if cell_value==2 {
                let one_forward=[ind[0],ind[1]-1];
                vec.push(one_forward);
                if self.turn_number==0 {
                    let two_forward =[ind[0],ind[1]-2];
                    vec.push(two_forward);
                }
            }
        }

        vec
    }

}
