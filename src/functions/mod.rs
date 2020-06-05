mod desk;
mod figura;
mod dependencies;
mod position;
mod reference;
mod diag;

use super::*;


pub fn cmp_vec(vec1: Vec<u8>, vec2: Vec<u8>) -> bool {
    let new_vec: Vec<u8> = vec1.iter().filter_map(|&x| vec2.iter().find(|&&y| x == y ))
        .map(|&z| z).collect();
    vec1.len() == vec2.len() && new_vec.len() == vec2.len() 
}

fn cmp_vec_of_vec(mut vec1: Vec<Vec<u8>>, vec2: Vec<Vec<u8>>) -> bool {

    println!("vec1: {:?}", &vec1);
    println!("vec2: {:?}", &vec2);
    let new_vec: Vec<&Vec<u8>> = vec1.iter().filter_map(|x| vec2.iter().find(|&y| x == y))
        .collect();
    vec1.len() == vec2.len() && new_vec.len() == vec2.len() 
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp_vec_of_vec() {
        assert!(cmp_vec_of_vec(vec![vec![0,1], vec![2,3]], vec![vec![2,3], vec![0,1]]));
        assert!(!cmp_vec_of_vec(vec![vec![1,0], vec![2,3]], vec![vec![2,3], vec![0,1]]));
    }
    
    #[test]
    fn test_cmp_vec_slice() {
        assert_eq!(&vec![1,2,3], &vec![1,2,3]);
        assert!(&vec![1,2,3] != &vec![3,2,1]);
        assert!(&vec![1,2] != &vec![3,2,1]);
        assert!(&vec![1,2] != &vec![1,2,4]);
    }
    
}
