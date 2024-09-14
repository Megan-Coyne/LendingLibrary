use std::collections::HashMap;

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
    for (title, book) in catalog {
        if book.is_available {
            println!("Available: {}", title);
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

// Example usage
fn main() {
    let mut library: HashMap<String, Book> = HashMap::new(); // Library catalog
    let mut user = User::new("Alice"); // Create a new user

    // Add books to the library
    add_book(&mut library, "The Catcher in the Rye", "J.D. Salinger");
    add_book(&mut library, "1984", "George Orwell");

    // List available books
    println!("Available books:");
    list_available_books(&library);

    // Borrow a book
    borrow_book(&mut user, &mut library, "1984");
    println!("\nAvailable books after borrowing:");
    list_available_books(&library);

    // Return a book
    return_book(&mut user, &mut library, "1984");
    println!("\nAvailable books after returning:");
    list_available_books(&library);
}
