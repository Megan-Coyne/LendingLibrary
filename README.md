
# Lending Library Catalog

This is a command-line lending library application built in Rust. The program tracks a catalog of books, their availability, and allows users to borrow and return books. Users can interact with the catalog dynamically by adding books, listing available books, and borrowing or returning books. The project demonstrates Rust’s ownership and borrowing principles, along with handling user input for a personalized experience.

## Features

- **Add Books**: Users can add books to the library catalog by providing the title and author.
- **List Available Books**: The application lists all books that are currently available to borrow, including their title and author.
- **Borrow Books**: Users can borrow books, which updates the book’s availability.
- **Return Books**: Users can return books they have borrowed, making them available in the catalog again.
- **Dynamic User Input**: Users input their name at the start, which is used throughout the program.
  
## Usage

### Prerequisites

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

### Running the Program

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/library-catalog.git
   cd library-catalog
   ```

2. Compile and run the program:

   ```bash
   cargo run
   ```

3. Interact with the program through the command-line interface. You will be prompted to:
   - Enter your name when the program starts.
   - Add books to the catalog, list available books, borrow, or return books using a menu.

### Example Workflow

```
Enter your name: Alice

Library Menu:
1. Add a book to the catalog
2. List available books
3. Borrow a book
4. Return a book
5. Exit
Choose an option (1-5): 1
Enter the book title: 1984
Enter the author: George Orwell
Added '1984' by George Orwell to the catalog.

Choose an option (1-5): 2
Available books:
1984 by George Orwell

Choose an option (1-5): 3
Enter the title of the book you want to borrow: 1984
Alice borrowed '1984' by George Orwell.
```

## Project Structure

- **Main Functionality**: Implemented in the `main.rs` file, where the `Book` and `User` structs are defined.
- **Book Struct**: Represents a book in the catalog with fields for title, author, and availability.
- **User Struct**: Tracks a user's name and the books they have borrowed.
- **Library Catalog**: Managed with a `HashMap<String, Book>` where the key is the book’s title, and the value is the corresponding `Book` struct.

## Contributions

Contributions are welcome! If you’d like to improve this project, feel free to open an issue or submit a pull request.

## License

This project is open-source and available under the [MIT License](LICENSE).
