use super::*;

impl Dependencies {
    pub fn new() -> Dependencies {
        Dependencies {
           dependencies: vec![] 
        }
    }
    pub fn insert(&mut self, reference: &mut Reference) {
        if let Some(find_referece_mut) = self.find_by_index_mut(reference.index_field) {
            find_referece_mut.depends_from.append(&mut reference.depends_from);
        } else {
            self.dependencies.push(reference.clone());
        }
    }
    pub fn insert_vec(&mut self, rederenceis: Vec<&mut Reference>) {

    }
    
    pub fn get_dependencies(&self, index: u8) -> Option<&Vec<u8>> {
        if let Some(find_reference) = self.find_by_index(index) {
            Some(&find_reference.depends_from)
        } else {
            None
        }
    }

    fn find_by_index(&self, index: u8) -> Option<&Reference> {
        self.dependencies.iter().find(|&&Reference{index_field: i,
            depends_from: _ }| i == index) 
    }

    fn find_by_index_mut(&mut self, index: u8) -> Option<&mut Reference> {
        self.dependencies.iter_mut().find(|&&mut Reference{index_field: i,
            depends_from: _ }| i == index) 
    }

}

#[cfg(test)]
mod tests {
    use super::*;

}
