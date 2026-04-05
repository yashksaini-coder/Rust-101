use std::collections::BTreeMap;

fn main() {
    // Create a new `BTreeMap`:
    let mut book_ratings = BTreeMap::new();

    // Insert some book ratings:
    book_ratings.insert("The Hitchhiker's Guide to the Galaxy", 3);
    book_ratings.insert("Pride and Prejudice", 4);
    book_ratings.insert("1984", 5);
    book_ratings.insert("To Kill a Mockingbird", 4);
    book_ratings.insert("Dune", 4);

    // Print the map (elements are printed in sorted key order):
    println!("Book ratings:");
    for (book, rating) in &book_ratings {
        println!("{book}: {rating}");
    }

    // Get the rating for a specific book:
    if let Some(rating) = book_ratings.get("1984") {
        println!("Rating for 1984: {rating}");
    }

    // Check if a book is in the map:
    if book_ratings.contains_key("Pride and Prejudice") {
        println!("Pride and Prejudice is in the map.");
    }

    // Remove a book and its rating:
    if let Some(rating) = book_ratings.remove("Dune") {
        println!("Removed Dune, rating was {rating}");
    }

    // Iterate over a range of books (lexicographically between "P" and "T"):
    println!("\nRatings for books between 'P' and 'R':");
    for (book, rating) in book_ratings.range("P".."R") {
        println!("{book}: {rating}");
    }

    // Find the first and last entries:
    println!("\nFirst entry: {:?}", book_ratings.first_key_value());
    println!("Last entry: {:?}", book_ratings.last_key_value());
    // Clear the map:
    book_ratings.clear();
    println!("Map is empty: {}", book_ratings.is_empty());
}