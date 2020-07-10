use super::*;

impl<'a> Position<'a> {

    pub fn all_figures(&self) -> Vec<(u8, &Figura)> {
        let mut res = vec![];
        for index in 0..64 {
            if let Some(figura) = self.get_figura(index) {
                res.push((index, figura));
            }
        }
        res
    }

    pub fn new(desk: &'a Desk, figura: &'a Figura) -> Position<'a> {
       let fictive_move = Move {from_cell: 64, to_cell:64, figura};
       let mut black_dep = Dependencies::new();
       let mut white_dep = Dependencies::new();

       for index in 0..64 {
           if let Some(fig) = desk.get_figura(index) {
               let possible_move = fig.get_possible_move2(index);
               for mut move1 in possible_move {
                   //println!("fig: {:?}, index: {:?}, move1: {:?}", fig, index, move1);
                   //для проверки ести ли препятствия на пути фигур
                   if fig.is_power() {
                       if let Some(index_first_fig) = move1.iter().position(|&x| desk.get_figura(x) != None) {
                           move1 = move1.iter().take(index_first_fig+1).map(|&x| x).collect();
                       }
                       /*
                           move1 = move1.iter().take_while(|&&x| desk.get_figura(x) == None)
                               .map(|&x| x).collect();
                               */
                       }
                       
//println!("move1: {:?}, index: {:?}", move1, index);
                   match fig.color {
                       Color::White => 
                           move1.iter().map(|&x| white_dep.insert(&mut Reference::new(x, index))).collect(),
                       Color::Black => 
                           move1.iter().map(|&x| black_dep.insert(&mut Reference::new(x, index))).collect(),
                   }
                   //println!("white_dep: {:?}", white_dep);
                   //println!("black_dep: {:?}", black_dep);
               }

           }
       }
       Position { previous: Previous::Desk(desk)
                , moves: fictive_move
                , white_dependencies: white_dep
                , black_dependencies: black_dep}
    }

    pub fn make_move(&'a self, moves: Move<'a>) -> Position<'a> {
        Position { previous: Previous::Position(self)
                 , moves: moves
                 , white_dependencies: Dependencies::new()
                 , black_dependencies: Dependencies::new()
        }
    }
    pub fn get_figura(&self, index_field: u8) -> Option<&'a Figura> {
        //проверяем есть ли наше поле в ходе откуда пошли, тогда сейчас это пустое поле
        if self.moves.from_cell == index_field {
            return None
        } else {
            //проверяем есть ли наше поле в ходе куда пошли, тогда возвращаем фигуру
            if self.moves.to_cell == index_field {
                return Some(self.moves.figura)
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

    fn get_king(&self, color_arg: Color) -> u8 {
        (0..64).into_iter().find(|&x| match self.get_figura(x) {
            Some(Figura {color: color_match, figura: FiguraType::King}) => color_match == &color_arg,
            _ => false
        }).unwrap()
        
    }

    fn get_all_posible_moves(&self) -> Vec<Move<'a>> {
        let dep_ref = match self.get_turn_color() {
            Color::White => &self.white_dependencies,
            Color::Black => &self.black_dependencies,
        };
        println!("dependencies: {:?}", dep_ref.dependencies);
        for ref1 in dep_ref.dependencies.iter() {
            println!("ref: {:?}", ref1);
            let move_vec: Vec<Move> = ref1.depends_from.iter().map(|&ind| Move { from_cell: ref1.index_field, 
                to_cell: ind, figura: self.get_figura(ind).unwrap() }).collect();
            println!("move: {:?}", move_vec);
        }
        return vec![]
    }

    fn get_turn_color(&self) -> Color {
        match self.previous {
            Previous::Position(position_ref) => !position_ref.get_turn_color(),
            Previous::Desk(desk_ref) => desk_ref.color_move
        }
    }

    fn is_shah(&self) -> bool {
        //find king
        let king_color = self.get_turn_color();
        let king_index = self.get_king(king_color);
        //в зависимостях найдем ссылки на это поле
        let enemy_depend = match king_color {
            Color::White => &self.black_dependencies.dependencies,
            Color::Black => &self.white_dependencies.dependencies,
        };
        println!("depend: {:?}", enemy_depend);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_king() {
        let desk1 = Desk::start_position();
        let position1 = Position::new(&desk1, desk1.get_figura(0).unwrap());
        assert_eq!(position1.get_king(Color::White), 4);
        assert_eq!(position1.get_king(Color::Black), 60);
    }

    #[test]
    fn test_get_all_moves() {
        let desk1 = Desk::position1();
        let position1 = Position::new(&desk1, desk1.get_figura(33).unwrap());
        let all_possible_move = position1.get_all_posible_moves();
        //так как пешка на 34 связана ладьей, то она не может ходить
        assert!(!all_possible_move.contains( &Move {from_cell: 34, to_cell: 42, 
            figura: desk1.get_figura(34).unwrap()} ));
        //а вот белый король может пойти на поле 42
        assert!(all_possible_move.contains( &Move { from_cell: 33, to_cell: 42, 
            figura: desk1.get_figura(33).unwrap() } ));
    }
    #[test]
    fn test_get_turn_color() {
        let desk1 = Desk::position1();
        let position1 = Position::new(&desk1, desk1.get_figura(33).unwrap());
        assert_eq!(position1.get_turn_color(), Color::White);
        let position2 = position1.make_move(Move {from_cell: 33, to_cell: 42,
            figura: position1.get_figura(33).unwrap()});
        assert_eq!(position2.get_turn_color(), Color::Black);
    }
    #[test]
    fn test_is_shah() {
        let desk1 = Desk::position1();
        let position1 = Position::new(&desk1, desk1.get_figura(33).unwrap());
        assert!(!position1.is_shah());
        let position2 = position1.make_move( Move {from_cell: 34, to_cell: 42,
            figura: position1.get_figura(34).unwrap()} );
        assert!(position2.is_shah());
    }
}
