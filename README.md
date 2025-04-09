# Library Management System

A simple command-line application for managing a collection of books, written in Rust.

## Features

- Add new books to your collection with title, author, ISBN, and publication year
- Search for books by title
- View a complete list of all books in your library
- Remove books by ISBN
- Persistent storage using JSON file

## Requirements

- Rust (latest stable version recommended)
- Serde crate for JSON serialization/deserialization

## Installation

1. Clone this repository or download the source code
2. Install Rust if you haven't already (https://www.rust-lang.org/tools/install)
3. Build the project:
   ```
   cargo build --release
   ```
4. Run the application:
   ```
   cargo run
   ```

## Usage

The application provides a simple menu-driven interface:

1. **Add a Book** - Add a new book with details (title, author, ISBN, publication year)
2. **Search for a Book by Title** - Find books with matching titles (case insensitive)
3. **List All Books** - Display all books in your library
4. **Remove a Book by ISBN** - Delete a book using its ISBN number
5. **Exit** - Close the application

## Data Storage

Your library is saved to a file named `library.json` in the same directory as the application. This file is automatically created when you add your first book and updated whenever changes are made.

## Technical Details

The application uses:
- `serde` for JSON serialization/deserialization
- Standard Rust file I/O operations for persistent storage
- A simple command-line interface for user interaction

## Project Structure

- `main.rs` - Contains all the application code including:
- `Book` struct definition
- Menu and user interaction functions
- Library data management functions
- File I/O operations

## Future Improvements

- Add book editing functionality
- Implement more search options (by author, ISBN, year)
- Add sorting and filtering capabilities
- Create a more user-friendly interface