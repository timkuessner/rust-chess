use macroquad::prelude::*;

use crate::board::Board;
use crate::piece::piece;

pub struct PieceTextures {
    white_pawn: Texture2D,
    white_knight: Texture2D,
    white_bishop: Texture2D,
    white_rook: Texture2D,
    white_queen: Texture2D,
    white_king: Texture2D,

    black_pawn: Texture2D,
    black_knight: Texture2D,
    black_bishop: Texture2D,
    black_rook: Texture2D,
    black_queen: Texture2D,
    black_king: Texture2D,
}

impl PieceTextures {
    pub async fn load() -> Self {
        async fn load_piece(name: &str) -> Texture2D {
            let path = format!("assets/pieces/{name}.png");
            let texture = load_texture(&path).await.unwrap();
            texture.set_filter(FilterMode::Nearest);
            texture
        }

        Self {
            white_pawn: load_piece("white_pawn").await,
            white_knight: load_piece("white_knight").await,
            white_bishop: load_piece("white_bishop").await,
            white_rook: load_piece("white_rook").await,
            white_queen: load_piece("white_queen").await,
            white_king: load_piece("white_king").await,

            black_pawn: load_piece("black_pawn").await,
            black_knight: load_piece("black_knight").await,
            black_bishop: load_piece("black_bishop").await,
            black_rook: load_piece("black_rook").await,
            black_queen: load_piece("black_queen").await,
            black_king: load_piece("black_king").await,
        }
    }

    fn get(&self, p: u8) -> Option<&Texture2D> {
        if p == piece::NONE {
            return None;
        }

        let is_white = piece::color(p) == piece::WHITE;

        let texture = match piece::kind(p) {
            piece::PAWN => {
                if is_white {
                    &self.white_pawn
                } else {
                    &self.black_pawn
                }
            }
            piece::KNIGHT => {
                if is_white {
                    &self.white_knight
                } else {
                    &self.black_knight
                }
            }
            piece::BISHOP => {
                if is_white {
                    &self.white_bishop
                } else {
                    &self.black_bishop
                }
            }
            piece::ROOK => {
                if is_white {
                    &self.white_rook
                } else {
                    &self.black_rook
                }
            }
            piece::QUEEN => {
                if is_white {
                    &self.white_queen
                } else {
                    &self.black_queen
                }
            }
            piece::KING => {
                if is_white {
                    &self.white_king
                } else {
                    &self.black_king
                }
            }
            _ => return None,
        };

        Some(texture)
    }
}

pub struct BoardLayout {
    pub offset_x: f32,
    pub offset_y: f32,
    pub square_size: f32,

    pub white_at_bottom: bool,
}

impl BoardLayout {
    pub fn new(white_at_bottom: bool) -> Self {
        let board_size = screen_height().min(screen_width()) * 0.8;
        Self {
            offset_x: (screen_width() - board_size) / 2.0,
            offset_y: (screen_height() - board_size) / 2.0,
            square_size: board_size / 8.0,
            white_at_bottom,
        }
    }

    pub fn board_size(&self) -> f32 {
        self.square_size * 8.0
    }

    pub fn square_origin(&self, file: usize, rank: usize) -> (f32, f32) {
        let (column, row) = if self.white_at_bottom {
            (file, 7 - rank)
        } else {
            (7 - file, rank)
        };

        (
            self.offset_x + column as f32 * self.square_size,
            self.offset_y + row as f32 * self.square_size,
        )
    }

    pub fn square_at(&self, (mx, my): (f32, f32)) -> Option<usize> {
        let fx = (mx - self.offset_x) / self.square_size;
        let fy = (my - self.offset_y) / self.square_size;

        if fx < 0.0 || fy < 0.0 || fx >= 8.0 || fy >= 8.0 {
            return None;
        }

        let (file, rank) = if self.white_at_bottom {
            (fx as usize, 7 - fy as usize)
        } else {
            (7 - fx as usize, fy as usize)
        };

        Some(rank * 8 + file)
    }
}

pub fn draw_board(
    board: &Board,
    textures: &PieceTextures,
    layout: &BoardLayout,
    last_move: Option<(usize, usize)>,
    dragging: Option<usize>,
    targets: &[usize],
    mouse: (f32, f32),
) {
    let square_size = layout.square_size;
    let highlight = Color::from_rgba(255, 255, 0, 110);
    let move_overlay = Color::from_rgba(220, 40, 40, 150);

    for rank in 0..8 {
        for file in 0..8 {
            let index = rank * 8 + file;

            let color = if (file + rank) % 2 == 0 {
                Color::from_rgba(181, 136, 99, 255)
            } else {
                Color::from_rgba(240, 217, 181, 255)
            };

            let (x, y) = layout.square_origin(file, rank);

            draw_rectangle(x, y, square_size, square_size, color);

            let is_last_move =
                matches!(last_move, Some((from, to)) if index == from || index == to);
            if is_last_move || dragging == Some(index) {
                draw_rectangle(x, y, square_size, square_size, highlight);
            }

            if targets.contains(&index) {
                draw_rectangle(x, y, square_size, square_size, move_overlay);
            }

            if dragging == Some(index) {
                continue;
            }

            if let Some(texture) = textures.get(board.squares[index]) {
                draw_texture_ex(
                    texture,
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(square_size, square_size)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    if let Some(from) = dragging {
        if let Some(texture) = textures.get(board.squares[from]) {
            draw_texture_ex(
                texture,
                mouse.0 - square_size / 2.0,
                mouse.1 - square_size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(square_size, square_size)),
                    ..Default::default()
                },
            );
        }
    }
}

pub struct PlayerPanel<'a> {
    pub name: &'a str,
    pub color: u8,

    pub clock: Option<f64>,

    pub active: bool,
}

pub fn draw_player_panel(layout: &BoardLayout, panel: &PlayerPanel, at_top: bool) {
    let font_size = (layout.square_size * 0.4).max(14.0);
    let padding = font_size * 0.4;
    let box_height = font_size + padding * 2.0;
    let gap = font_size * 0.3;

    let y = if at_top {
        layout.offset_y - box_height - gap
    } else {
        layout.offset_y + layout.board_size() + gap
    };
    let baseline = y + padding + font_size * 0.8;

    let marker_radius = font_size * 0.35;
    let marker_x = layout.offset_x + marker_radius;
    let marker_y = y + box_height / 2.0;

    if panel.color == piece::WHITE {
        draw_circle(marker_x, marker_y, marker_radius, WHITE);
    } else {
        draw_circle_lines(marker_x, marker_y, marker_radius, 1.5, WHITE);
    };

    draw_text(
        panel.name,
        layout.offset_x + marker_radius * 2.0 + padding,
        baseline,
        font_size,
        WHITE,
    );

    if let Some(seconds) = panel.clock {
        let text = format_clock(seconds);
        let width = measure_text(&text, None, font_size as u16, 1.0).width;
        let box_width = width + padding * 2.0;
        let box_x = layout.offset_x + layout.board_size() - box_width;

        let (background, text_color) = match (panel.active, seconds < 20.0) {
            (true, true) => (RED, WHITE),
            (true, false) => (WHITE, BLACK),
            (false, _) => (BLACK, WHITE),
        };

        draw_rectangle(box_x, y, box_width, box_height, background);
        draw_text(&text, box_x + padding, baseline, font_size, text_color);
    }
}

fn format_clock(seconds: f64) -> String {
    let seconds = seconds.max(0.0);

    if seconds < 10.0 {
        return format!("0:{:04.1}", seconds);
    }

    let total = seconds.ceil() as u64;
    let (hours, minutes, secs) = (total / 3600, (total / 60) % 60, total % 60);

    if hours > 0 {
        format!("{hours}:{minutes:02}:{secs:02}")
    } else {
        format!("{minutes}:{secs:02}")
    }
}

pub fn draw_result_banner(title: &str, subtitle: &str) {
    let width = screen_width();
    let height = screen_height();
    let unit = width.min(height);

    let title_size = unit * 0.07;
    let subtitle_size = unit * 0.04;

    let box_width = (measure_text(subtitle, None, subtitle_size as u16, 1.0).width + unit * 0.12)
        .max(unit * 0.6);
    let box_height = unit * 0.3;
    let box_x = (width - box_width) / 2.0;
    let box_y = (height - box_height) / 2.0;

    draw_rectangle(
        box_x,
        box_y,
        box_width,
        box_height,
        Color::from_rgba(0, 0, 0, 128),
    );
    draw_rectangle_lines(box_x, box_y, box_width, box_height, 3.0, WHITE);

    let centered = |text: &str, size: f32, y: f32, color: Color| {
        let text_width = measure_text(text, None, size as u16, 1.0).width;
        draw_text(text, (width - text_width) / 2.0, y, size, color);
    };

    centered(title, title_size, box_y + box_height * 0.36, WHITE);
    centered(subtitle, subtitle_size, box_y + box_height * 0.62, WHITE);
}
