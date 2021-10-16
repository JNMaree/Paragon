#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::geometry::base::{*};
use std::vec;

pub struct pUID {
    pub id: u64,
    pub str: String
}

pub struct pUID_Register {
    register: Vec<pUID>,
    n_entries: pCount
}

impl Default for pUID_Register {
    fn default() -> Self {
        return pUID_Register {
            register: Vec::new(),
            n_entries: 0
        }
    }
}

impl pUID_Register {

    pub fn init(&self) {

    }

    pub fn add_UID(&mut self, uid: &pUID) {

    }

    pub fn del_UID(&mut self, index: pCount) {
        let del_ind = self.register.iter().position(|x| *x == index).unwrap();
        self.register.remove(del_ind);
    }

    pub fn get_UID(&self, index: pCount) -> pUID {
        if index < self.n_entries {
            return self.register[index];
        } else {
            panic!("Specified Index:{} does not exist in register.")
        }

    }
}