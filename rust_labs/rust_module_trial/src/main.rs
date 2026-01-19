mod db;
mod services;

use db::entities::{
    Cat, Dog
};

use services::{
    handler::handler,
    custom_types::AnimalKind
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create input data
    let kind: AnimalKind = AnimalKind::Dog;
    let dog: Dog = Dog { name: "Bob".to_string() };
    let cat: Cat = Cat { name: "Lisa".to_string() };

    let animal_handler = handler::new(None, Some(dog)); // None for cat

    // handle the dog
    animal_handler.say_name(kind)?;
    
    Ok(())
}
