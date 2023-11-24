pub struct Group {
    id: super::Id,
    elems: Vec<super::Id>,
}

impl Group {
    pub fn new(id: super::Id) -> Self {
        Self {
            id,
            elems: Vec::new(),
        }
    }
    pub fn id(&self) -> &super::Id {
        &self.id
    }

    pub fn elems(&self) -> &Vec<super::Id> {
        &self.elems
    }

    /// Push an element id, if the id is already in the list, returns an Err(())
    pub fn push(&mut self, id: super::Id) -> Result<(), ()> {
        if self.elems.contains(&id) {
            return Err(());
        }
        self.elems.push(id);
        Ok(())
    }

    /// Removes an element id, if the given id is not saved, returns an Err(())
    pub fn remove(&mut self, id: super::Id) -> Result<(), ()> {
        if !self.elems.contains(&id) {
            return Err(());
        }
        let index = self.elems.iter().position(|e| *e == id).unwrap();
        self.elems.remove(index);
        Ok(())
    }
}
