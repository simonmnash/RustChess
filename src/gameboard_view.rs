//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;

use GameboardController;

/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub black_background_color: Color,
    /// White tile background color.
    pub white_background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
    /// Color of Selected Cell.
    pub selected_cell_background_color: Color,
    pub legal_move_background_color: Color,
    /// Text color.
    pub text_color: Color,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 400.0,
            black_background_color: [0.5, 0.5, 0.5, 1.0],
            white_background_color: [0.9, 0.9, 0.9, 0.9],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.4, 0.4, 1.0, 1.0],
            legal_move_background_color: [0.7, 0.7, 1.0, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0]
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            settings: settings,
        }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics, C>(
        &self,
        controller: &GameboardController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G)
            where C: CharacterCache<Texture = G::Texture>
    {
        use graphics::{Image, Line, Rectangle, Transformed};

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];


        // Draw board background.
        let black_tile= Rectangle::new(settings.black_background_color);

        let white_tile = Rectangle::new(settings.white_background_color);
        white_tile.draw(board_rect, &c.draw_state, c.transform, g);

        for i in 0..8 {
            for j in 0..8 {
                let black_rect = [
                    settings.position[0]+(settings.size/8.0*j as f64), settings.position[1]+(settings.size/8.0*i as f64),
                    settings.size/8.0, settings.size/8.0
                ];
                if (i+j)%2!=0 {
                    black_tile.draw(black_rect, &c.draw_state, c.transform, g);
                }

            }
        }
        // Color selected cell.
        if let Some (ind) = controller.selected_cell {

            let cell_size = settings.size/8.0;
            let pos = [ind[0] as f64 * cell_size, ind[1] as f64 * cell_size];
            let cell_rect = [
                settings.position[0]+pos[0],settings.position[1]+pos[1], cell_size, cell_size
            ];
            Rectangle::new(settings.selected_cell_background_color)
                .draw(cell_rect, &c.draw_state,c.transform, g);

            let rects_to_color=&controller.legal_moves;
            for i in rects_to_color {
                let pos = [i[0] as f64 * cell_size, i[1] as f64 * cell_size];
                let cell_rect = [
                    settings.position[0]+pos[0],settings.position[1]+pos[1], cell_size, cell_size
                ];
                Rectangle::new(settings.legal_move_background_color)
                    .draw(cell_rect, &c.draw_state,c.transform, g);
            }
        }

        // Draw characters.
        let text_image = Image::new_color(settings.text_color);
        let cell_size = settings.size / 8.0;
        for j in 0..8 {
            for i in 0..8 {
                if let Some(ch) = controller.gameboard.char([i, j]) {
                    let pos = [
                        settings.position[0] + i as f64 * cell_size + 15.0,
                        settings.position[1] + j as f64 * cell_size + 34.0
                        ];
                        let character = glyphs.character(34, ch);
                        let ch_x = pos[0] + character.left();
                        let ch_y = pos[1] - character.top();
                        text_image.draw(character.texture,
                                &c.draw_state,
                                c.transform.trans(ch_x, ch_y),
                                g);
                }
            }
        }

        // Draw cell borders.
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        for i in 0..8 {
            let x = settings.position[0] + i as f64 / 8.0 * settings.size;
            let y = settings.position[1] + i as f64 / 8.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);
    }
}
