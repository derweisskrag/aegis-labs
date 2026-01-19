use crate::db::entities::{
    Cat, 
    Dog
};

use super::custom_types::AnimalKind;

pub struct handler {
    pub cat: Option<Cat>,
    pub dog: Option<Dog>
}

impl handler {
    pub fn new(cat: Option<Cat>, dog: Option<Dog>) -> Self {
        Self {
            cat: Some(cat).expect("Cat must exist"),
            dog: Some(dog).expect("Dog must exist")
        }
    }

    pub fn say_name(&self, kind: AnimalKind) -> Result<(), Box<dyn std::error::Error>> {
        match kind {
            AnimalKind::Cat => {
                if let Some(cat) = &self.cat {
                    println!("{}", cat.name);
                } else {
                    println!("No cat found");
                }
            },

            AnimalKind::Dog => {
                if let Some(dog) = &self.dog {
                    println!("{}", dog.name);
                } else {
                    println!("No dog found");
                }
            }
        };

        Ok(())
    }
}