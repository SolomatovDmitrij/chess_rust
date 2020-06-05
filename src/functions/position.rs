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
                   println!("fig: {:?}, index: {:?}, move1: {:?}", fig, index, move1);
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
                       
println!("move1: {:?}, index: {:?}", move1, index);
                   match fig.color {
                       Color::White => 
                           move1.iter().map(|&x| white_dep.insert(&mut Reference::new(x, index))).collect(),
                       Color::Black => 
                           move1.iter().map(|&x| black_dep.insert(&mut Reference::new(x, index))).collect(),
                   }
                   println!("white_dep: {:?}", white_dep);
                   println!("black_dep: {:?}", black_dep);
               }

           }
       }
               /*
                      match desk.get_figura(index) {
               //white pawn
               Some(Figura {color: Color::White, figura: FiguraType::Pawn}) => {
                   //проверим ошибку 
                   if Desk::in_gorizont(index, 7) { 
                       panic!("белая пешка не может находится на последней горизонтале");
                   }
                   white_dep.insert(&mut Reference::new(index+8, index));
                   //если пешка на второй линии и нет преграды тогда она может ходить на 2 клетки
                   if Desk::in_gorizont(index, 1) && desk.get_figura(index+8).is_none() {
                       white_dep.insert(&mut Reference::new(index+16, index));
                   } 
               },
               //Black pawn
               Some(Figura {color: Color::Black, figura: FiguraType::Pawn}) => {
                   //проверим ошибку 
                   if Desk::in_gorizont(index, 0) { 
                       panic!("черная пешка не может находится на первой горизонтале");
                   }
                   black_dep.insert(&mut Reference::new(index-8, index));
                   //если пешка на второй линии и нет преграды тогда она может ходить на 2 клетки
                   if Desk::in_gorizont(index, 6) && desk.get_figura(index-8).is_none() {
                       black_dep.insert(&mut Reference::new(index-16, index));
                   } 
               },
               //King any color
               Some(Figura {color, figura: FiguraType::King}) => {
                   let mut dep = &mut black_dep;
                   if *color == Color::White {
                       dep = &mut white_dep;
                   }

                   //если это не 0 вертикаль то король может сходить на лево
                   if !Desk::in_vertical(index, 0) {
                       dep.insert(&mut Reference::new(index-1,index));

                       if !Desk::in_gorizont(index, 0) {
                           dep.insert(&mut Reference::new(index-9,index));
                       }
                       if !Desk::in_gorizont(index, 7) {
                           dep.insert(&mut Reference::new(index+7,index));
                       }
                   }
                   if !Desk::in_vertical(index, 7) {
                       dep.insert(&mut Reference::new(index+1,index));

                       if !Desk::in_gorizont(index, 0) {
                           dep.insert(&mut Reference::new(index-7,index));
                       }
                       if !Desk::in_gorizont(index, 7) {
                           dep.insert(&mut Reference::new(index+9,index));
                       }
                   }
                   if !Desk::in_gorizont(index, 0) {
                       dep.insert(&mut Reference::new(index-8,index));
                   }
                   if !Desk::in_gorizont(index, 7) {
                       dep.insert(&mut Reference::new(index+8,index));
                   }
               },
               //Knight any color
               Some(Figura {color, figura: FiguraType::Knight}) => {
                   let mut dep = &mut black_dep;
                   if *color == Color::White {
                       dep = &mut white_dep;
                   }

                   //если это не 0 вертикаль то конь  может сходить на лево
                   if !Desk::in_vertical(index, 0) {
                       if !Desk::in_gorizont(index, 0) && !Desk::in_gorizont(index, 1) {
                           dep.insert(&mut Reference::new(index-9,index));
                       }
                       if !Desk::in_gorizont(index, 7) {
                           dep.insert(&mut Reference::new(index+7,index));
                       }
                   }
                   if !Desk::in_vertical(index, 7) {
                       dep.insert(&mut Reference::new(index+1,index));

                       if !Desk::in_gorizont(index, 0) {
                           dep.insert(&mut Reference::new(index-7,index));
                       }
                       if !Desk::in_gorizont(index, 7) {
                           dep.insert(&mut Reference::new(index+9,index));
                       }
                   }
                   if !Desk::in_gorizont(index, 0) {
                       dep.insert(&mut Reference::new(index-8,index));
                   }
                   if !Desk::in_gorizont(index, 7) {
                       dep.insert(&mut Reference::new(index+8,index));
                   }
               },

               _ => {} //continue
           

           }*/
       
       /*
       println!("white-dep: {:?}", white_dep);
       println!("black-dep: {:?}", black_dep);
       */
       /*
       let depend = (0..64).filter_map(|x| {
           match desk.get_figura(x) {
               Some(f) => Some(x, f),
               None => None
           }
       }).map(|fig|{
               match fig {
                   Some()
                       
               }
           })
       })
       */
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
/*
    pub fn get_dependence(&self, index_field: u8) -> Option<&Reference> {
       self.dependencies.iter().find(|&x| x.index_field == index_field)
    }
  */  
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
}

