use super::*;

impl Desk {
    pub fn get_figura(&self, index_figura: u8) -> Option<&Figura> {
        match &self.cells[usize::from(index_figura)] {
            None => None,
            Some(fig) => Some(&fig)
        }
    }
    pub fn get_goriz_index(index: u8) -> u8 {
        index / 8
    }
    pub fn get_vert_index(index: u8) -> u8 {
        index % 8
    }
    pub fn in_gorizont(index: u8, goriz: u8 ) -> bool {
        Desk::get_goriz_index(index) == goriz
    }
    pub fn in_vertical(index: u8, vert: u8 ) -> bool {
        Desk::get_vert_index(index) == vert
    }
    pub fn in_gorizont_vec(index: i8, goriz: Vec<i8> ) -> bool {
        if index < 0 || index > 63 {
            return false
        }
        goriz.contains(&(Desk::get_goriz_index(index as u8) as i8))
          
    }
    pub fn in_vertical_vec(index: i8, vert: Vec<i8> ) -> bool {
        if index < 0 || index > 63 {
            return false
        }

        vert.contains(&(Desk::get_vert_index(index as u8) as i8))
    }
    pub fn get_left_diag_index(index: u8) -> u8 {
        Desk::get_vert_index(index) + Desk::get_goriz_index(index)
    }
    pub fn get_right_diag_index(index: u8) -> i8 {
        Desk::get_vert_index(index) as i8 - Desk::get_goriz_index(index) as i8 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_in_vert_vec() {
        assert!(Desk::in_vertical_vec(57, vec![1]));
        assert!(Desk::in_vertical_vec(63, vec![7]));
        assert!(Desk::in_vertical_vec(16, vec![0]));

        let vert = Desk::get_vert_index(63) as i8;
        assert_eq!(Desk::in_vertical_vec(57, vec![vert-2, vert-1, vert, vert+1, vert+2]), false);
    }

    #[test]
    fn test_in_gorizont_vec() {
        assert!(Desk::in_gorizont_vec(0, vec![1, 0, 2]));
        assert!(Desk::in_gorizont_vec(32, vec![4]));
        assert!(Desk::in_gorizont_vec(63, vec![7]));

        assert!(!Desk::in_gorizont_vec(0, vec![1, 2]));
        assert!(!Desk::in_gorizont_vec(32, vec![-1, 1]));
        assert!(!Desk::in_gorizont_vec(63, vec![6]));

    }
    #[test]
    fn test_get_figura() {
        //получим начальную позицию
        let desk = Desk::start_position();
        //ход конем с b1 на c1
        let knight_ref = &desk.get_figura(1).unwrap();
        let move_b1_c3 = Move {from_cell: 1, to_cell: 18, figura: knight_ref};
        let start_position = Position::new(&desk, knight_ref);
        let position = start_position.make_move(move_b1_c3);
        
        //проверка на поле с3 должен быть белый конь
        assert_eq!(*position.get_figura(18).unwrap(), Figura::new(Color::White, FiguraType::Knight));
        assert_eq!(position.get_figura(1), None);

    }

    #[test]
    fn test_get_vert_index() {
        assert_eq!(Desk::get_vert_index(0), 0);
        assert_eq!(Desk::get_vert_index(63), 7);
        assert_eq!(Desk::get_vert_index(35), 3);
    }

    #[test]
    fn test_get_gor_index() {
        assert_eq!(Desk::get_goriz_index(0), 0);
        assert_eq!(Desk::get_goriz_index(24), 3);
        assert_eq!(Desk::get_goriz_index(59), 7);
    }

    #[test]
    fn test_get_figura_desk() {
        let desk = Desk::start_position();
        assert_eq!(*desk.get_figura(0).unwrap(), Figura::new(Color::White, FiguraType::Rook));
    }
    #[test]
    fn test_in_goriz() {
        assert!(Desk::in_gorizont(0, 0));
        assert!(Desk::in_gorizont(10, 1));
        assert!(Desk::in_gorizont(60, 7));
    }
    #[test]
    fn test_in_vert() {
        assert!(Desk::in_vertical(0, 0));
        assert!(Desk::in_vertical(41, 1));
        assert!(Desk::in_vertical(23, 7));
    }
    #[test]
    fn test_get_left_diag_index() {
        assert_eq!(Desk::get_left_diag_index(0), 0);
        assert_eq!(Desk::get_left_diag_index(1), 1);
        assert_eq!(Desk::get_left_diag_index(17), 3);
    }
    #[test]
    fn test_get_right_diag_index() {
        assert_eq!(Desk::get_right_diag_index(9), 0);
        assert_eq!(Desk::get_right_diag_index(8), -1);
        assert_eq!(Desk::get_right_diag_index(2), 2);
    }
    #[test]
    fn test_insert_dependencies() {
        let mut new_dep = Dependencies::new();
        new_dep.insert(&mut Reference::new(1, 1));
        new_dep.insert(&mut Reference::new(1, 2));
        new_dep.insert(&mut Reference::new(1, 3));
        assert_eq!(*new_dep.get_dependencies(1).unwrap(),vec![1,2,3]);
    }
}
