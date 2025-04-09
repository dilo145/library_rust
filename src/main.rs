use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    year: u32,
}

const LIBRARY_FILE: &str = "library.json";

fn main() {
    let mut library = load_library();

    loop {
        println!("\n************ Menu ************ Library Management ************");
        println!("1. Add a Book");
        println!("2. Search for a Book by Title");
        println!("3. List All Books");
        println!("4. Remove a Book by ISBN");
        println!("5. Exit");
        println!("***************************************************************");

        save_library(&library);

        let choice = read_input("Enter your choice: ");
        match choice.trim() {
            "1" => add_book(&mut library),
            "2" => search_book(&library),
            "3" => list_books(&library),
            "4" => remove_book(&mut library),
            "5" => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn add_book(library: &mut Vec<Book>) {
    println!("--- Add a New Book ---");
    let title = read_input("Title: ");
    let author = read_input("Author: ");
    let isbn = read_input("ISBN: ");
    let year_input = read_input("Year of Publication: ");

    match year_input.trim().parse::<u32>() {
        Ok(year) => {
            let book = Book {
                title,
                author,
                isbn,
                year,
            };
            library.push(book);
            println!("Book added successfully!");
        }
        Err(_) => println!("Invalid year format."),
    }
}

fn search_book(library: &Vec<Book>) {
    println!("--- Search for a Book ---");
    let title = read_input("Enter the title to search for: ");
    let mut found = false;

    for book in library {
        if book.title.to_lowercase().contains(&title.to_lowercase()) {
            println!("{:?}", book);
            found = true;
        }
    }

    if !found {
        println!("No books found with that title.");
    }
}

fn list_books(library: &Vec<Book>) {
    println!("--- List of All Books ---");
    if library.is_empty() {
        println!("The library is currently empty.");
    } else {
        for (i, book) in library.iter().enumerate() {
            println!("{}. {:?}", i + 1, book);
        }
    }
}

fn remove_book(library: &mut Vec<Book>) {
    println!("--- Remove a Book ---");
    let isbn = read_input("Enter the ISBN of the book to remove: ");
    let before = library.len();

    library.retain(|book| book.isbn != isbn);

    if library.len() < before {
        println!("Book removed successfully!");
    } else {
        println!("No book found with that ISBN.");
    }
}

fn load_library() -> Vec<Book> {
    if Path::new(LIBRARY_FILE).exists() {
        let content = fs::read_to_string(LIBRARY_FILE).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_library(library: &Vec<Book>) {
    let data = serde_json::to_string_pretty(library).unwrap();
    fs::write(LIBRARY_FILE, data).expect("Error saving library to file.");
}
