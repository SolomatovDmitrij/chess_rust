use super::*;

//вспомогательные локальные функции 
fn get_top(index: u8) -> Vec<u8> {
    let mut result = Vec::new();

    let mut new_ind = index+8;
    while new_ind < 64 {
        result.push(new_ind);
        new_ind += 8;
    }
    result
}
fn get_bottom(index: u8) -> Vec<u8> {
    let mut result = Vec::new();

    let mut new_ind = index as i8 -8;
    while new_ind >= 0 {
        result.push(new_ind as u8);
        new_ind -= 8;
    }
    result
}
fn get_left(index: u8) -> Vec<u8> {
    let mut result = Vec::new();

    let gor = Desk::get_goriz_index(index);

    let mut new_ind = (index as i8) - 1;
    while new_ind >= 0 && gor == Desk:: get_goriz_index(new_ind as u8) {
        result.push(new_ind as u8);
        new_ind -= 1;
    }
    result
}
fn get_right(index: u8) -> Vec<u8> {
    let mut result = Vec::new();
    let gor = Desk::get_goriz_index(index);
    let mut new_ind = index + 1;
    while new_ind < 64 && gor == Desk::get_goriz_index(new_ind as u8) {
        result.push(new_ind);
        new_ind += 1;
    }
    result
}
fn get_left_top(index: u8) -> Vec<u8> {
    let mut result = Vec::new();
    let left_diag_index = Desk::get_left_diag_index(index);
    let mut new_ind = index + 7;
    while new_ind < 64 && left_diag_index == Desk::get_left_diag_index(new_ind as u8){
        result.push(new_ind);
        new_ind += 7;
    }
    result
}
fn get_left_bottom(index: u8) -> Vec<u8> {
    let mut result = Vec::new();
    let diag = Desk::get_right_diag_index(index);
    let mut new_ind = index as i8 - 9;
    while new_ind >= 0 && diag == Desk::get_right_diag_index(new_ind as u8) {
        result.push(new_ind as u8);
        new_ind -= 9;
    }
    result
}
fn get_right_top(index: u8) -> Vec<u8> {
    let mut result = Vec::new();
    let diag = Desk::get_right_diag_index(index);
    let mut new_ind = index + 9;
    while new_ind < 64 && diag == Desk::get_right_diag_index(new_ind) {
        result.push(new_ind);
        new_ind += 9;
    }
    result
}
fn get_right_bottom(index: u8) -> Vec<u8> {
    let mut result = Vec::new();
    let diag = Desk::get_left_diag_index(index);
    let mut new_ind = index as i8 - 7;
    while new_ind >= 0 && diag == Desk::get_left_diag_index(new_ind as u8) {
        result.push(new_ind as u8);
        new_ind -= 7;
    }
    result
}

impl Diag {
    pub fn new(index: u8) -> Diag {
        Diag {
            top: get_top(index),
            bottom: get_bottom(index),
            left: get_left(index),
            right: get_right(index),
            left_top: get_left_top(index),
            right_top: get_right_top(index),
            left_bottom: get_left_bottom(index),
            right_bottom: get_right_bottom(index),
        }
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_new_diag() {
        let diag = Diag::new(45);

        assert_eq!(diag.left, vec![44, 43, 42, 41, 40]);
        assert_eq!(diag.right, vec![46, 47]);
        assert_eq!(diag.top, vec![53, 61]);
        assert_eq!(diag.bottom, vec![37, 29, 21, 13, 5]);

        assert_eq!(diag.left_top, vec![52, 59]);
        assert_eq!(diag.right_top, vec![54, 63]);
        assert_eq!(diag.left_bottom, vec![36, 27, 18, 9, 0]);
        assert_eq!(diag.right_bottom, vec![38, 31]);
    }

    #[test]
    fn test_get_top() {
        assert_eq!(get_top(39), vec![47, 55, 63]);
    }
    #[test]
    fn test_get_bottom() {
        assert_eq!(get_bottom(16), vec![8, 0]);
        assert_eq!(get_bottom(17), vec![9, 1]);
        assert_eq!(get_bottom(15), vec![7]);
    }
    #[test]
    fn test_get_left() {
        assert_eq!(get_left(2), vec![1,0]);
        assert_eq!(get_left(9), vec![8]);
    }
    #[test]
    fn test_get_right() {
        assert_eq!(get_right(61), vec![62, 63]);
        assert_eq!(get_right(53), vec![54, 55]);
    }
    #[test]
    fn test_get_left_top() {
        assert_eq!(get_left_top(10), vec![17, 24]);
        assert_eq!(get_left_top(47), vec![54, 61]);
    }
    #[test]
    fn test_get_right_top() {
        assert_eq!(get_right_top(45), vec![54,63]);
        assert_eq!(get_right_top(21), vec![30, 39]);
    }
    #[test]
    fn test_get_left_bottom() {
        assert_eq!(get_left_bottom(18), vec![9, 0]);
        assert_eq!(get_left_bottom(34), vec![25, 16]);
    }
    #[test]
    fn test_get_right_bottom() {
        assert_eq!(get_right_bottom(37), vec![30, 23]);
        assert_eq!(get_right_bottom(21), vec![14, 7]);
    }
}
