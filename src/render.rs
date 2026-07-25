use ratatui::{Frame, style::{Color, Style}, text::{Line, Span}};

use crate::state::{GameState, HEIGHT, HIDDEN_HEIGHT, PieceKind, WIDTH};

pub fn draw(state: &GameState, frame: &mut Frame) {
    let mut vec_outer: Vec<Line> = vec![];
    for r in HIDDEN_HEIGHT..HEIGHT {
        let mut vec_inner: Vec<Span> = vec![];
        for c in 0..WIDTH {
            let cell = state.board.cells[r][c];
            let span = match cell {
                Some(kind) => Span::styled("██", Style::default().fg(color(kind))),
                None => Span::raw("  "),
            };

            vec_inner.push(span);
        }
        vec_outer.push(Line::from(vec_inner))
    }
}

pub fn color(kind: PieceKind) -> Color {
    match kind {
        PieceKind::I => Color::Cyan,
        PieceKind::O => Color::Yellow,
        PieceKind::T => Color::Magenta,
        PieceKind::S => Color::Green,
        PieceKind::Z => Color::Red,
        PieceKind::J => Color::Blue,
        PieceKind::L => Color::Rgb(255, 196, 0),
    }
}
