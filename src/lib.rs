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
    from_cell: u8,
    to_cell: u8, 
    figura: &'a Figura,
}

#[derive(Clone, Debug)]
struct Reference {
    index_field: u8,
    depends_from: Vec<u8>,
}

struct Position<'a> {
    previous: Previous<'a>,
    white_dependencies: Dependencies,
    black_dependencies: Dependencies,
    moves: Move<'a>,
}

type FiguraVector<'a> = Vec<(u8, &'a Figura)>;

trait FindFigura {
    fn get_by_index(&self, index: u8) -> Option<&Figura>;
}

#[derive(Debug)]
struct Dependencies {
    dependencies: Vec<Reference>,
}

struct Diag {
    left: Vec<u8>,
    right: Vec<u8>,
    top: Vec<u8>,
    bottom: Vec<u8>,
    left_top: Vec<u8>,
    left_bottom: Vec<u8>,
    right_top: Vec<u8>,
    right_bottom: Vec<u8>,
}
