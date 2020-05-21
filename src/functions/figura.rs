use super::*;

impl Figura {
    fn get_possible_move(&self, position: &Position) -> Vec<Move> {
        match self {
            Figura {color, figura: FiguraType::Pawn} => {
                vec![]
            },
            _ => vec![]
        }
    }
}

impl<'a> Position<'a> {
    fn get_possible_move(&self, index_of_field: usize) -> Vec<Move<'a>> {
        let figura1 = &self.get_figura(index_of_field).unwrap();
        println!("figura: {:?}", figura1);
        match figura1 {
            Figura {color: Color::Black, figura: FiguraType::Pawn} => {
                let move1 = Move {from_cell: index_of_field, to_cell: (index_of_field - 8, figura1)};
                vec![move1]
            },
            _ => vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_moves_from_figura() {
        let desk = Desk::position1();
        let move_b5_b6 = Move {from_cell: 33, to_cell: (41, &desk.get_figura(33).unwrap())};
        let position1 = desk.new_position(move_b5_b6);
        let move_h4_h3 = Move {from_cell: 23, to_cell: (15, position1.get_figura(23).unwrap())};
        assert_eq!(position1.get_figura(23).unwrap().get_possible_move(&position1), vec![move_h4_h3]);
    }

    #[test]
    fn test_get_moves_from_position() {
        let desk = Desk::position1();
        let move_b5_b6 = Move {from_cell: 33, to_cell: (41, &desk.get_figura(33).unwrap())};
        let position1 = desk.new_position(move_b5_b6);
        let move_h4_h3 = Move {from_cell: 23, to_cell: (15, position1.get_figura(23).unwrap())};
        assert_eq!(position1.get_possible_move(23), vec![move_h4_h3]);
    }
}


