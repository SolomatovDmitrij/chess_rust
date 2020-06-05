use super::*;

impl Reference {
    pub fn new(index: u8, value: u8) -> Reference {
        Reference {index_field: index, depends_from: vec![value]}
    }
    pub fn new_vec(index: u8, value: Vec<u8>) -> Reference {
        Reference {index_field: index, depends_from: value}
    }
}


