use polodb_core::Database;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    print_books()?;

    add_books()?;

    // print_books()?;

    Ok(())
}

fn print_books() -> Result<(), Box<dyn Error>> {
    let db = Database::open_file("books.db")?;
    let collection = db.collection::<Book>("books");

    let books = collection.find(None)?;

    for book in books {
        println!("name: {:?}", book);
    }

    Ok(())
}

fn add_books() -> Result<(), Box<dyn Error>> {
    let docs = vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            title: "Animal Farm".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
        },
    ];

    let db = Database::open_file("books.db")?;
    let collection = db.collection::<Book>("books");

    collection.insert_many(docs)?;

    Ok(())
}
