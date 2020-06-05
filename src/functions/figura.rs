use super::*;

impl FindFigura for FiguraVector<'_> {
    fn get_by_index(&self, index: u8) -> Option<&Figura> {
        if let Some((_ ,y)) = self.iter().find(|&(x, _)| *x == index) {
            return Some(y)
        }
        None
    }
}

impl Figura {
    pub fn new(color: Color, figura: FiguraType) -> Figura {
        Figura {color, figura}
    }

    //для таких фигур как слон, ладья, или ферзь нужна отдельная функция
    pub fn get_possible_move2(&self, index: u8) -> Vec<Vec<u8>> {
        match self {
            //слон
            Figura {color: _, figura: FiguraType::Bishop} => {
                let diag = Diag::new(index);
                vec![diag.left_top, diag.right_top, diag.left_bottom, diag.right_bottom]
                    .into_iter().filter(|x| !x.is_empty() ).collect()
            },
            //ладья
            Figura {color: _, figura: FiguraType::Rook} => {
                let diag = Diag::new(index);
                vec![diag.left, diag.right, diag.top, diag.bottom]
                    .into_iter().filter(|x| !x.is_empty()).collect()
            },
            //ферзь
            Figura {color: _, figura: FiguraType::Queen} => {
                let diag = Diag::new(index);
                vec![diag.left, diag.right, diag.top, diag.bottom, 
                    diag.left_top, diag.right_top, diag.left_bottom, diag.right_bottom]
                    .into_iter().filter(|x| !x.is_empty()).collect()
            },
            _ => vec![self.get_possible_move(index)]
        }

    }

    fn get_possible_move(&self, index: u8) -> Vec<u8> {
        let ind = index as i8;
        let gor = Desk::get_goriz_index(index) as i8;
        let vert = Desk::get_vert_index(index) as i8;
        match self {
            //Король
            Figura {color: _, figura: FiguraType::King} => {
                vec![ind-1, ind+1, ind-7, ind-8, ind-9, ind+7, ind+8, ind+9]
                    .into_iter().filter(|&x| Desk::in_gorizont_vec(x, vec![gor-1, gor, gor+1]) && 
                                        Desk::in_vertical_vec(x, vec![vert-1, vert, vert+1]))
                    .map(|x| x as u8)
                    .collect()
            },
            //черная пешка
            Figura {color: Color::Black, figura: FiguraType::Pawn} => {
               if gor == 7 || gor == 0 {
                   panic!("черная пешка не может быть на первой или последней горизонтале");
               } 
               if gor == 6 {
                   vec![index - 8, index - 16]
               } else {
                   vec![index - 8]
               }
            },
            //белая пешка
            Figura {color: Color::White, figura: FiguraType::Pawn} => {
               if gor == 7 || gor == 0 {
                   panic!("Пешка не может быть на первой или последней горизонтале");
               } 
               if gor == 1 {
                   vec![index + 8, index + 16]
               } else {
                   vec![index + 8]
               }
            },
            //конь
            Figura {color: _, figura: FiguraType::Knight} => {
                vec![ind+6, ind+10, ind+15, ind+17, ind-6, ind-10, ind-15, ind-17]
                    .into_iter().filter(|&x| Desk::in_gorizont_vec(x, vec![gor-2, gor-1, gor, gor+1, gor+2]) && 
                                        Desk::in_vertical_vec(x, vec![vert-2, vert-1, vert, vert+1, vert+2]))
                    .map(|x| x as u8)
                    .collect()

            },
            _ => vec![]
        }
    }
    pub fn is_power(&self) -> bool {
        match self.figura {
            FiguraType::Queen | FiguraType::Bishop | FiguraType::Rook => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_figura_power() {
        assert_eq!(Figura::new(Color::White, FiguraType::Rook).is_power(), true);
        assert_eq!(Figura::new(Color::White, FiguraType::Queen).is_power(), true);
        assert_eq!(Figura::new(Color::White, FiguraType::Bishop).is_power(), true);
        assert_eq!(Figura::new(Color::White, FiguraType::Pawn).is_power(), false);
        assert_eq!(Figura::new(Color::White, FiguraType::Knight).is_power(), false);
        assert_eq!(Figura::new(Color::White, FiguraType::King).is_power(), false);
    }

    #[test]
    fn test_cmp_vec() {
        assert!(cmp_vec(vec![1,2,3], vec![2,3,1]));
//        println!("cmp_res: {:?}", cmp_vec(vec![1,2], vec![1,2,3]));
        assert!(!cmp_vec(vec![1,2,3], vec![1,2]));
        assert!(!cmp_vec(vec![2,3], vec![1,2]));
    }

    #[test]
    fn test_get_moves_from_figura_king() {
        println!("fig possible move 32: {:?}", Figura::new(Color::White, FiguraType::King).get_possible_move(32));
        assert!(cmp_vec(Figura::new(Color::White, FiguraType::King).get_possible_move(0), vec![8,9,1]));
        assert!(cmp_vec(Figura::new(Color::Black, FiguraType::King).get_possible_move(32), 
                        vec![40,41,33,25,24]));
        assert!(cmp_vec(Figura::new(Color::Black, FiguraType::King).get_possible_move(35), 
                        vec![42,43,44,36,28,27,26,34]));

    }

    #[test]
    fn test_get_move_pawn() {
        assert_eq!(Figura::new(Color::White, FiguraType::Pawn).get_possible_move(8), vec![16,24]);
        assert_eq!(Figura::new(Color::White, FiguraType::Pawn).get_possible_move(19), vec![27]);
        assert_eq!(Figura::new(Color::Black, FiguraType::Pawn).get_possible_move(55), vec![47,39]);
        assert_eq!(Figura::new(Color::Black, FiguraType::Pawn).get_possible_move(36), vec![28]);
    }
    
    #[test]
    fn test_knight_move() {
        assert!(cmp_vec(Figura::new(Color::White, FiguraType::Knight).get_possible_move(0), 
                         vec![17,10]));
        assert!(cmp_vec(Figura::new(Color::White, FiguraType::Knight).get_possible_move(63), 
                         vec![53,46]));
        assert!(cmp_vec(Figura::new(Color::White, FiguraType::Knight).get_possible_move(36), 
                         vec![42,51,53,46,30,21,19,26]));
    }

    #[test]
    fn test_bishop_move() {
        assert_eq!(Figura::new(Color::White, FiguraType::Bishop).get_possible_move2(35),
                    Figura::new(Color::Black, FiguraType::Bishop).get_possible_move2(35));
        assert!(cmp_vec_of_vec(Figura::new(Color::White, FiguraType::Bishop).get_possible_move2(0), 
                        vec![vec![9,18,27,36,45,54,63]]  ));
        assert!(cmp_vec_of_vec(Figura::new(Color::Black, FiguraType::Bishop).get_possible_move2(42),
        vec![vec![49,56], vec![51,60], vec![35, 28, 21, 14 ,7], vec![33, 24]]));
    }
    #[test]
    fn test_rook_move() {
        assert!(cmp_vec_of_vec(Figura::new(Color::White, FiguraType::Rook).get_possible_move2(0),
            vec![vec![8,16,24,32,40,48,56], vec![1,2,3,4,5,6,7]]
        ));
        assert!(cmp_vec_of_vec(Figura::new(Color::White, FiguraType::Rook).get_possible_move2(63),
            vec![vec![62,61,60,59,58,57,56], vec![55,47,39,31,23,15,7]]
        ));
        assert!(cmp_vec_of_vec(Figura::new(Color::White, FiguraType::Rook).get_possible_move2(36),
            vec![vec![44,52,60], vec![37,38,39], vec![35,34,33,32], vec![28,20,12,4]]
        ));
    }
    #[test]
    fn test_queen_move() {
        assert!(cmp_vec_of_vec(Figura::new(Color::White, FiguraType::Queen).get_possible_move2(0),
            vec![vec![8,16,24,32,40,48,56], vec![9,18,27,36,45,54,63], vec![1,2,3,4,5,6,7]]));
        assert!(cmp_vec_of_vec(Figura::new(Color::Black, FiguraType::Queen).get_possible_move2(35),
            vec![vec![42,49,56], vec![43,51,59], vec![44,53,62], vec![36,37,38,39], vec![28,21,14,7],
                vec![27,19,11,3], vec![26,17,8], vec![34,33,32]]));
    }

    //тест  расстановки зависимостей
    #[test]
    fn test_dependencies() {
        let desk = Desk::position1();
        let position1 = Position::new(&desk, desk.get_figura(33).unwrap());
        println!("position1.white_dependencies: {:?}", position1.white_dependencies);
        let dependence_42 = position1.white_dependencies.get_dependencies(42).unwrap();
        assert!(dependence_42.contains(&33) && dependence_42.contains(&34));
        //тест начальной позиции
        let desk_start = Desk::start_position();
        let start_position = Position::new(&desk_start, desk_start.get_figura(0).unwrap());
        //ладья а1 не имеет зависимостей и т.д. 
        //Белые
        //первый ряд
        assert_eq!(start_position.white_dependencies.get_dependencies(0), None);
        assert!(start_position.white_dependencies.get_dependencies(1).unwrap().contains(&0));
        assert!(start_position.white_dependencies.get_dependencies(2).unwrap().contains(&3));
        assert!(start_position.white_dependencies.get_dependencies(3).unwrap().contains(&4));
        assert!(start_position.white_dependencies.get_dependencies(4).unwrap().contains(&3));
        assert!(start_position.white_dependencies.get_dependencies(5).unwrap().contains(&4));
        assert!(start_position.white_dependencies.get_dependencies(6).unwrap().contains(&7));
        assert_eq!(start_position.white_dependencies.get_dependencies(7), None);

        //второй ряд
        assert!(start_position.white_dependencies.get_dependencies(8).unwrap().contains(&0));
        assert!(start_position.white_dependencies.get_dependencies(9).unwrap().contains(&2));
        assert!(start_position.white_dependencies.get_dependencies(10).unwrap().contains(&3));
        assert!(start_position.white_dependencies.get_dependencies(11).unwrap().contains(&2));
        assert!(start_position.white_dependencies.get_dependencies(11).unwrap().contains(&3));
        assert!(start_position.white_dependencies.get_dependencies(11).unwrap().contains(&1));
        assert!(start_position.white_dependencies.get_dependencies(11).unwrap().contains(&4));
        assert!(start_position.white_dependencies.get_dependencies(12).unwrap().contains(&3));
        assert!(start_position.white_dependencies.get_dependencies(12).unwrap().contains(&4));
        assert!(start_position.white_dependencies.get_dependencies(12).unwrap().contains(&5));
        assert!(start_position.white_dependencies.get_dependencies(12).unwrap().contains(&6));
        assert!(start_position.white_dependencies.get_dependencies(13).unwrap().contains(&4));
        assert!(start_position.white_dependencies.get_dependencies(14).unwrap().contains(&5));
        assert!(start_position.white_dependencies.get_dependencies(15).unwrap().contains(&7));

        //третий ряд
        assert!(start_position.white_dependencies.get_dependencies(16).unwrap().contains(&8));
        assert!(start_position.white_dependencies.get_dependencies(16).unwrap().contains(&1));
        assert!(start_position.white_dependencies.get_dependencies(17).unwrap().contains(&9));
        assert!(start_position.white_dependencies.get_dependencies(18).unwrap().contains(&10));
        assert!(start_position.white_dependencies.get_dependencies(18).unwrap().contains(&1));
        assert!(start_position.white_dependencies.get_dependencies(19).unwrap().contains(&11));
        assert!(start_position.white_dependencies.get_dependencies(20).unwrap().contains(&12));
        assert!(start_position.white_dependencies.get_dependencies(21).unwrap().contains(&6));
        assert!(start_position.white_dependencies.get_dependencies(21).unwrap().contains(&13));
        assert!(start_position.white_dependencies.get_dependencies(22).unwrap().contains(&14));
        assert!(start_position.white_dependencies.get_dependencies(23).unwrap().contains(&15));
        assert!(start_position.white_dependencies.get_dependencies(23).unwrap().contains(&6));

        //четвертый ряд
        assert!(start_position.white_dependencies.get_dependencies(24).unwrap().contains(&8));
        assert!(start_position.white_dependencies.get_dependencies(25).unwrap().contains(&9));
        assert!(start_position.white_dependencies.get_dependencies(26).unwrap().contains(&10));
        assert!(start_position.white_dependencies.get_dependencies(27).unwrap().contains(&11));
        assert!(start_position.white_dependencies.get_dependencies(28).unwrap().contains(&12));
        assert!(start_position.white_dependencies.get_dependencies(29).unwrap().contains(&13));
        assert!(start_position.white_dependencies.get_dependencies(30).unwrap().contains(&14));
        assert!(start_position.white_dependencies.get_dependencies(31).unwrap().contains(&15));

//Черные
//первый ряд
        assert_eq!(start_position.black_dependencies.get_dependencies(56), None);
        assert!(start_position.black_dependencies.get_dependencies(57).unwrap().contains(&56));
        assert!(start_position.black_dependencies.get_dependencies(58).unwrap().contains(&59));
        assert!(start_position.black_dependencies.get_dependencies(59).unwrap().contains(&60));
        assert!(start_position.black_dependencies.get_dependencies(60).unwrap().contains(&59));
        assert!(start_position.black_dependencies.get_dependencies(61).unwrap().contains(&60));
        assert!(start_position.black_dependencies.get_dependencies(62).unwrap().contains(&63));
        assert_eq!(start_position.black_dependencies.get_dependencies(63), None);

        //второй ряд
        assert!(start_position.black_dependencies.get_dependencies(48).unwrap().contains(&56));
        assert!(start_position.black_dependencies.get_dependencies(49).unwrap().contains(&58));
        assert!(start_position.black_dependencies.get_dependencies(50).unwrap().contains(&59));
        assert!(start_position.black_dependencies.get_dependencies(51).unwrap().contains(&58));
        assert!(start_position.black_dependencies.get_dependencies(51).unwrap().contains(&59));
        assert!(start_position.black_dependencies.get_dependencies(51).unwrap().contains(&60));
        assert!(start_position.black_dependencies.get_dependencies(51).unwrap().contains(&57));
        assert!(start_position.black_dependencies.get_dependencies(52).unwrap().contains(&59));
        assert!(start_position.black_dependencies.get_dependencies(52).unwrap().contains(&60));
        assert!(start_position.black_dependencies.get_dependencies(52).unwrap().contains(&61));
        assert!(start_position.black_dependencies.get_dependencies(52).unwrap().contains(&62));
        assert!(start_position.black_dependencies.get_dependencies(53).unwrap().contains(&60));
        assert!(start_position.black_dependencies.get_dependencies(54).unwrap().contains(&61));
        assert!(start_position.black_dependencies.get_dependencies(55).unwrap().contains(&63));

        //третий ряд
        assert!(start_position.black_dependencies.get_dependencies(40).unwrap().contains(&48));
        assert!(start_position.black_dependencies.get_dependencies(40).unwrap().contains(&57));
        assert!(start_position.black_dependencies.get_dependencies(41).unwrap().contains(&49));
        assert!(start_position.black_dependencies.get_dependencies(42).unwrap().contains(&57));
        assert!(start_position.black_dependencies.get_dependencies(42).unwrap().contains(&50));
        assert!(start_position.black_dependencies.get_dependencies(43).unwrap().contains(&51));
        assert!(start_position.black_dependencies.get_dependencies(44).unwrap().contains(&52));
        assert!(start_position.black_dependencies.get_dependencies(45).unwrap().contains(&62));
        assert!(start_position.black_dependencies.get_dependencies(45).unwrap().contains(&53));
        assert!(start_position.black_dependencies.get_dependencies(46).unwrap().contains(&54));
        assert!(start_position.black_dependencies.get_dependencies(47).unwrap().contains(&55));
        assert!(start_position.black_dependencies.get_dependencies(47).unwrap().contains(&62));

        //четвертый ряд
        assert!(start_position.black_dependencies.get_dependencies(32).unwrap().contains(&48));
        assert!(start_position.black_dependencies.get_dependencies(33).unwrap().contains(&49));
        assert!(start_position.black_dependencies.get_dependencies(34).unwrap().contains(&50));
        assert!(start_position.black_dependencies.get_dependencies(35).unwrap().contains(&51));
        assert!(start_position.black_dependencies.get_dependencies(36).unwrap().contains(&52));
        assert!(start_position.black_dependencies.get_dependencies(37).unwrap().contains(&53));
        assert!(start_position.black_dependencies.get_dependencies(38).unwrap().contains(&54));
        assert!(start_position.black_dependencies.get_dependencies(39).unwrap().contains(&55));




    }

    //тест на получение всех фигур с доски
    #[test]
    fn test_all_figures() {
        let desk = Desk::position1();
        let position1 = Position::new(&desk, desk.get_figura(33).unwrap()); 
        let all_figures = position1.all_figures();
        assert_eq!(*all_figures.get_by_index(33).unwrap(), Figura::new(Color::White, FiguraType::King));
    }
}


