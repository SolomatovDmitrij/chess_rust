use super::*;

impl Figura {
    fn new(color: Color, figura: FiguraType) -> Figura {
        Figura {color, figura}
    }
}

impl<'a> Position<'a> {
    pub fn new(&'a self, moves: Move<'a>) -> Position<'a> {
        Position { previous: Previous::Position(self)
                 , moves: moves }//Move {from_cell: moves.from_cell, to_cell: moves.to_cell} }
    }

    pub fn get_figura(&self, index_field: usize) -> Option<&'a Figura> {
        //проверяем есть ли наше поле в ходе откуда пошли, тогда сейчас это пустое поле
        if self.moves.from_cell == index_field {
            return None
        } else {
            //проверяем есть ли наше поле в ходе куда пошли, тогда возвращаем фигуру
            let (index_to_cell, figura_to_cell) = self.moves.to_cell;
            if index_to_cell == index_field {
                return Some(figura_to_cell)
            } else {
                //текущий ход нас не устроил проверяем предыдущую позицию
                match &self.previous {
                    Previous::Desk(desk) => desk.get_figura(index_field),
                    Previous::Position(position) => position.get_figura(index_field)
                }
            }
        }
        
    }

    fn value(&self) -> f32 {
        0.0
    }
}

impl Desk {
    pub fn get_figura(&self, index_figura: usize) -> Option<&Figura> {
        match &self.cells[index_figura] {
            None => None,
            Some(fig) => Some(&fig)
        }
    }
    pub fn new_position<'a>(&'a self, moves: Move<'a>) -> Position<'a> {
        Position {previous:  Previous::Desk(self), moves}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_figura() {
        //получим начальную позицию
        let desk = Desk::start_position();
        //ход конем с b1 на c1
        let move_b1_c3 = Move {from_cell: 1, to_cell: (18, &desk.get_figura(1).unwrap())};
        let position = Position {previous: Previous::Desk(&desk), moves: move_b1_c3};
        //проверка на поле с3 должен быть белый конь
        assert_eq!(*position.get_figura(18).unwrap(), Figura::new(Color::White, FiguraType::Knight));
        assert_eq!(position.get_figura(1), None);

    }

    #[test]
    fn test_position_value() {
        let desk = Desk::start_position();
        let move_b1_c3 = Move {from_cell: 1, to_cell: (18, &desk.get_figura(1).unwrap())};
        let start_position = desk.new_position(move_b1_c3);
        assert_eq!(start_position.value(), 0.0);
    }

    #[test]
    fn test_get_figura_desk() {
        let desk = Desk::start_position();
        assert_eq!(*desk.get_figura(0).unwrap(), Figura::new(Color::White, FiguraType::Rook));
    }
}
