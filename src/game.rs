use std::time::Duration;

use macroquad::prelude::*;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

use crate::board::Board;
use crate::move_generator::{DrawReason, MoveGeneration, MoveGenerator};
use crate::moves::{Move, MoveFlag};
use crate::piece::piece;
use crate::render::{self, BoardLayout, PieceTextures, PlayerPanel};

pub trait Player: Send {
    fn name(&self) -> &str;

    fn is_human(&self) -> bool;

    fn choose_move(&mut self, _board: &Board, _legal_moves: &[Move]) -> Option<Move> {
        None
    }
}

pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Player for HumanPlayer {
    fn name(&self) -> &str {
        &self.name
    }

    fn is_human(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Checkmate { winner: u8 },
    Timeout { winner: u8 },
    Draw(DrawReason),
}

pub struct Game {
    board: Board,
    textures: PieceTextures,

    players: Vec<Option<Box<dyn Player>>>,
    player_names: Vec<String>,
    bot_receiver: Option<Receiver<(usize, Box<dyn Player>, Option<Move>)>>,

    time_increment: Option<Duration>,

    clocks: [f64; 2],
    flip_board_each_move: bool,

    bottom_color: u8,

    legal_moves: Vec<Move>,
    result: Option<GameResult>,

    dragging: Option<usize>,
    selected: Option<usize>,
    targets: Vec<usize>,
    last_move: Option<(usize, usize)>,
}

impl Game {
    pub async fn new() -> Self {
        let mut game = Self {
            board: Board::new(),
            textures: PieceTextures::load().await,
            players: Vec::with_capacity(2),
            player_names: Vec::with_capacity(2),
            bot_receiver: None,
            time_increment: None,
            clocks: [0.0; 2],
            flip_board_each_move: false,
            bottom_color: piece::WHITE,
            legal_moves: Vec::new(),
            result: None,
            dragging: None,
            selected: None,
            targets: Vec::new(),
            last_move: None,
        };

        game.update_legal_moves();
        game
    }

    pub fn add_player(&mut self, player: impl Player + 'static) -> &mut Self {
        assert!(self.players.len() < 2, "a game only has two players");

        self.player_names.push(player.name().to_owned());
        self.players.push(Some(Box::new(player)));

        self
    }

    pub fn set_time(&mut self, initial: Duration, increment: Duration) -> &mut Self {
        self.time_increment = Some(increment);
        self.clocks = [initial.as_secs_f64(); 2];
        self
    }

    pub fn set_flip_board_each_move(&mut self, flip: bool) -> &mut Self {
        self.flip_board_each_move = flip;
        self.update_perspective();
        self
    }

    pub fn update(&mut self) -> bool {
        if is_key_pressed(KeyCode::Escape) {
            return false;
        }

        if self.is_ready() && self.result.is_none() {
            self.tick_clock();

            if self.result.is_none() {
                self.handle_turn();
            }
        }

        self.draw();

        true
    }

    fn is_ready(&self) -> bool {
        self.players.len() == 2
    }

    fn tick_clock(&mut self) {
        if self.time_increment.is_none() {
            return;
        }

        let color = self.board.color_to_move;
        let index = piece::color_index(color);

        self.clocks[index] -= get_frame_time() as f64;

        if self.clocks[index] <= 0.0 {
            self.clocks[index] = 0.0;

            self.finish(if self.board.has_insufficient_material() {
                GameResult::Draw(DrawReason::InsufficientMaterial)
            } else {
                GameResult::Timeout {
                    winner: piece::opposite(color),
                }
            });
        }
    }

    fn handle_turn(&mut self) {

        if let Some(receiver) = &self.bot_receiver {
            match receiver.try_recv() {
                Ok((index, player, mv)) => {
                    self.players[index] = Some(player);
                    self.bot_receiver = None;

                    if let Some(mv) = mv {
                        if self.legal_moves.contains(&mv) {
                            self.play_move(mv);
                        }
                    }

                    return;
                }
                Err(TryRecvError::Empty) => return,
                Err(TryRecvError::Disconnected) => {
                    self.bot_receiver = None;
                    return;
                }
            }
        }

        let index = piece::color_index(self.board.color_to_move);

        if self.players[index].as_ref().unwrap().is_human() {
            self.handle_human_input();
            return;
        }


        let mut player = self.players[index].take().unwrap();

        let board = self.board.clone();
        let legal_moves = self.legal_moves.clone();

        let (sender, receiver) = mpsc::channel();

        self.bot_receiver = Some(receiver);

        thread::spawn(move || {
            let mv = player.choose_move(&board, &legal_moves);

            let _ = sender.send((index, player, mv));
        });
    }

    fn handle_human_input(&mut self) {
        let layout = BoardLayout::new(self.bottom_color == piece::WHITE);
        let mouse = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(to) = layout.square_at(mouse) {
                if let (Some(from), true) = (self.selected, self.targets.contains(&to)) {
                    self.play_from_to(from, to);
                } else if self.is_own_piece(to) {
                    self.dragging = Some(to);
                    self.selected = Some(to);
                    self.targets = legal_move_targets(&self.legal_moves, to);
                } else {
                    self.selected = None;
                    self.targets.clear();
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(from) = self.dragging.take() {
                if let Some(to) = layout.square_at(mouse) {
                    if to != from && self.targets.contains(&to) {
                        self.play_from_to(from, to);
                    }
                }
            }
        }
    }

    fn is_own_piece(&self, square: usize) -> bool {
        let p = self.board.squares[square];

        p != piece::NONE && piece::color(p) == self.board.color_to_move
    }

    fn play_from_to(&mut self, from: usize, to: usize) {
        if let Some(mv) = find_move(&self.legal_moves, from, to) {
            self.play_move(mv);
        }
    }

    fn play_move(&mut self, mv: Move) {
        let mover = piece::color_index(self.board.color_to_move);

        self.board.make_move(mv);
        self.last_move = Some((mv.from, mv.to));

        if let Some(increment) = self.time_increment {
            self.clocks[mover] += increment.as_secs_f64();
        }

        self.dragging = None;
        self.selected = None;
        self.targets.clear();

        self.update_legal_moves();
    }

    fn update_legal_moves(&mut self) {
        match MoveGenerator::generate_moves(&self.board) {
            MoveGeneration::Moves(moves) => {
                self.legal_moves = moves;
                self.update_perspective();
            }
            MoveGeneration::Checkmate { winner } => {
                self.finish(GameResult::Checkmate {
                    winner: if winner { piece::WHITE } else { piece::BLACK },
                });
            }
            MoveGeneration::Draw(reason) => self.finish(GameResult::Draw(reason)),
        }
    }

    fn update_perspective(&mut self) {
        if self.result.is_none() {
            self.bottom_color = if self.flip_board_each_move {
                self.board.color_to_move
            } else {
                piece::WHITE
            };
        }
    }

    fn finish(&mut self, result: GameResult) {
        self.legal_moves.clear();
        self.dragging = None;
        self.selected = None;
        self.targets.clear();
        self.result = Some(result);
    }

    fn player_name(&self, color: u8) -> &str {
        self.player_names
            .get(piece::color_index(color))
            .map(String::as_str)
            .unwrap_or(if color == piece::WHITE {
                "White"
            } else {
                "Black"
            })
    }

    fn panel(&self, color: u8) -> PlayerPanel<'_> {
        PlayerPanel {
            name: self.player_name(color),
            color,
            clock: self
                .time_increment
                .map(|_| self.clocks[piece::color_index(color)]),
            active: self.is_ready() && self.result.is_none() && self.board.color_to_move == color,
        }
    }

    fn draw(&self) {
        clear_background(BLACK);

        let layout = BoardLayout::new(self.bottom_color == piece::WHITE);

        render::draw_board(
            &self.board,
            &self.textures,
            &layout,
            self.last_move,
            self.dragging,
            &self.targets,
            mouse_position(),
        );

        render::draw_player_panel(
            &layout,
            &self.panel(piece::opposite(self.bottom_color)),
            true,
        );
        render::draw_player_panel(&layout, &self.panel(self.bottom_color), false);

        if !self.is_ready() {
            render::draw_result_banner("Waiting for players", "Add two players to start the game");
        } else if let Some(result) = self.result {
            let (title, subtitle) = self.describe_result(result);
            render::draw_result_banner(&title, &subtitle);
        }
    }

    fn describe_result(&self, result: GameResult) -> (String, String) {
        let winner_text = |winner: u8| {
            let side = if winner == piece::WHITE {
                "White"
            } else {
                "Black"
            };
            format!("{} ({side}) wins", self.player_name(winner))
        };

        match result {
            GameResult::Checkmate { winner } => ("Checkmate".into(), winner_text(winner)),
            GameResult::Timeout { winner } => ("Out of time".into(), winner_text(winner)),
            GameResult::Draw(reason) => (
                "Draw".into(),
                match reason {
                    DrawReason::Stalemate => "Draw by stalemate",
                    DrawReason::ThreefoldRepetition => "Draw by threefold repetition",
                    DrawReason::FiftyMoveRule => "Draw by the fifty-move rule",
                    DrawReason::InsufficientMaterial => "Draw by insufficient material",
                }
                .into(),
            ),
        }
    }
}

fn find_move(legal_moves: &[Move], from: usize, to: usize) -> Option<Move> {
    legal_moves
        .iter()
        .find(|m| {
            m.from == from
                && m.to == to
                && !matches!(
                    m.flag,
                    MoveFlag::PromoteToKnight | MoveFlag::PromoteToRook | MoveFlag::PromoteToBishop
                )
        })
        .copied()
}

fn legal_move_targets(legal_moves: &[Move], from: usize) -> Vec<usize> {
    let mut targets: Vec<usize> = legal_moves
        .iter()
        .filter(|m| m.from == from)
        .map(|m| m.to)
        .collect();

    targets.sort();
    targets.dedup();

    targets
}
