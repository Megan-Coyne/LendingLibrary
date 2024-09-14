use std::{collections::HashMap, io::{self, Write}};

// Struct to represent a Book
struct Book {
    title: String,
    author: String,
    is_available: bool,
}

// Struct to represent a User
struct User {
    name: String,
    borrowed_books: Vec<String>,
}

impl User {
    // Create a new User
    fn new(name: &str) -> User {
        User {
            name: name.to_string(),
            borrowed_books: Vec::new(),
        }
    }
}

// Add a new book to the library catalog
fn add_book(catalog: &mut HashMap<String, Book>, title: &str, author: &str) {
    let book = Book {
        title: title.to_string(),
        author: author.to_string(),
        is_available: true,
    };
    catalog.insert(title.to_string(), book);
}

// List all available books in the catalog
fn list_available_books(catalog: &HashMap<String, Book>) {
    println!("Available books:");
    for (_title, book) in catalog {
        if book.is_available {
            println!("{} by {}", book.title, book.author);
        }
    }
}

// Borrow a book from the library
fn borrow_book(user: &mut User, catalog: &mut HashMap<String, Book>, book_title: &str) {
    if let Some(book) = catalog.get_mut(book_title) {
        if book.is_available {
            // Book is available, mark as borrowed
            book.is_available = false;
            user.borrowed_books.push(book_title.to_string());
            println!("{} borrowed '{}'", user.name, book_title);
        } else {
            println!("Book '{}' is not available", book_title);
        }
    } else {
        println!("Book '{}' not found in the catalog", book_title);
    }
}

// Return a book to the library
fn return_book(user: &mut User, catalog: &mut HashMap<String, Book>, book_title: &str) {
    if let Some(book) = catalog.get_mut(book_title) {
        if let Some(pos) = user
            .borrowed_books
            .iter()
            .position(|title| title == book_title)
        {
            // Remove the book from the user's borrowed list
            user.borrowed_books.remove(pos);
            book.is_available = true;
            println!("{} returned '{}'", user.name, book_title);
        } else {
            println!("{} has not borrowed '{}'", user.name, book_title);
        }
    } else {
        println!("Book '{}' not found in the catalog", book_title);
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensures the prompt is displayed before waiting for input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string() // Removes any trailing newlines or spaces
}

// Example usage
fn main() {
    let mut library: HashMap<String, Book> = HashMap::new();

    println!("Welcome to the Lending Library Program!");

    let user_name = get_input("Enter your name: ");
    let mut user = User::new(&user_name);

    loop {
        // Display menu options
        println!("\nLibrary Menu:");
        println!("1. Add a book to the catalog");
        println!("2. List available books");
        println!("3. Borrow a book");
        println!("4. Return a book");
        println!("5. Exit");

        // Get user's choice
        let choice = get_input("Choose an option (1-5): ");

        match choice.as_str() {
            "1" => {
                // Add a book to the catalog
                let title = get_input("Enter the book title: ");
                let author = get_input("Enter the author: ");
                add_book(&mut library, &title, &author);
                println!("Added '{}' by {} to the catalog.", title, author);
            }
            "2" => {
                // List available books
                list_available_books(&library);
            }
            "3" => {
                // Borrow a book
                let book_title = get_input("Enter the title of the book you want to borrow: ");
                borrow_book(&mut user, &mut library, &book_title);
            }
            "4" => {
                // Return a book
                let book_title = get_input("Enter the title of the book you want to return: ");
                return_book(&mut user, &mut library, &book_title);
            }
            "5" => {
                // Exit the program
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please choose a number between 1 and 5.");
            }
        }
    }
}
