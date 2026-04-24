use std::collections::LinkedList;

/// Macro to create a LinkedList with initial values.
/// Example: `let list = list![1, 2, 3];`
macro_rules! list {
    () => {
        LinkedList::new()
    };
    ($($item:expr),+ $(,)?) => {
        {
            let mut temp_list = LinkedList::new();
            $(
                temp_list.push_back($item);
            )+
            temp_list
        }
    };
}

/// Macro to print a debug message with context.
/// Example: `debug_println!("Value: {}", x);`
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[DEBUG] {}", format!($($arg)*));
    };
}

/// Application holding a linked list of integers.
/// Uses Rust's standard `LinkedList` instead of a custom node structure.
struct App {
    pub storage: LinkedList<i32>,
}

impl App {
    /// Creates a new App with an empty list.
    fn new() -> Self {
        Self {
            storage: LinkedList::new(),
        }
    }

    /// Adds a value to the end of the list.
    /// (Rust's LinkedList supports both push_front and push_back.)
    fn add_to_front(&mut self, value: i32) {
        self.storage.push_back(value);
    }

    /// Prints all values in the list using the built‑in iterator.
    fn print_list(&self) {
        if self.storage.is_empty() {
            println!("List is empty.");
            return;
        }

        println!("List contents:");
        for elem in self.storage.iter() {
            println!("{}", elem);
        }
    }

    
    
}

/// Factory‑style setup function.
/// Returning Result allows future expansion (config loading, IO, etc.)
fn set_up_app() -> Result<App, Box<dyn std::error::Error>> {
    Ok(App::new())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    debug_println!("Starting application...");
    
    let mut my_app = set_up_app()?;

    my_app.add_to_front(1);
    my_app.add_to_front(2);
    my_app.add_to_front(3);

    my_app.print_list();

    // Demonstrate the list! macro
    debug_println!("Creating a list with the list! macro");
    let _sample_list = list![10, 20, 30, 40];

    Ok(())
}


#[test]
fn test_list_macro() {
    let list = list![1,2,3];
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
}
