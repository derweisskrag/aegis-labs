#[derive(Clone, Debug)]
pub struct Cat {
    pub name: String
}

#[derive(Clone, Debug)]
pub struct Dog {
    pub name: String
}

impl Cat {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }

    pub fn say_name(&self) -> &str { // Returns a reference to a string slice
        &self.name
    }

    pub fn introduce_itself(&self) {
        let name = self.say_name(); // Get the reference
        println!("I am cat and my name is {}", name);
    }
}

impl Dog {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }

    pub fn introduce_itself(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("I am a dog and my name is {}", &self.name);
        Ok(())
    }
}