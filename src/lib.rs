mod positions;
mod functions;

#[derive (PartialEq, Debug, Clone)]
pub enum Color {
    Black,
    White,
}

#[derive (PartialEq, Debug, Clone)]
pub enum FiguraType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    King,
    Queen,
}
 
enum Previous<'a> {
    Position(&'a Position<'a>),
    Desk(&'a Desk),
}

#[derive (PartialEq, Debug, Clone)]
pub struct Figura {
    pub color: Color,
    pub figura: FiguraType,
}

struct Desk {
    cells: [Option<Figura>; 64],
    additional_figures: Vec<Figura>,
}

#[derive (PartialEq, Debug, Clone)]
struct Move<'a> {
    from_cell: usize,
    to_cell: (usize, &'a Figura)
}

struct Position<'a> {
    previous: Previous<'a>,
    moves: Move<'a>,
}
