use std::collections::HashMap;

// anything that can exist across time in
// multiple time periods
trait Temporal {
    fn get_timeline(&self) -> &Timeline;
    fn exists_at(&self, year: u32) -> bool;
}

// items that can be borrowed must also implement
// Temporal - as in, they must be borrowed at some point
// in time
trait Borrowable: Temporal {
    fn is_available_at(&self, year: u32) -> bool;
    fn checkout_at(&mut self, year: u32) -> Result<(), String>;
    fn return_at(&mut self, year: u32) -> Result<(), String>;
}
struct Timeline {
    // year could be signed, to allow for BC years
    start_year: u32,
    end_year: Option<u32>
}
// a book that can exist across time
struct Book<'a> {
    title: String,
    author: &'a str,
    timeline: Timeline,
    checkout_history: HashMap<u32, bool>
}

impl<'a> Book<'a> {
    fn new(title: String, author: &'a str, start_year: u32, end_year: Option<u32>) -> Self {
        Book {
            title,
            author,
            timeline: Timeline { start_year, end_year },
            checkout_history: HashMap::new(),
        }
    }
}

impl<'a> Temporal for Book<'a> {
    fn get_timeline(&self) -> &Timeline {
        &self.timeline
    }

    fn exists_at(&self, year: u32) -> bool {
        year >= self.timeline.start_year && match self.timeline.end_year {
            Some(end) => year <= end,
            None => true
        }
    }
}

impl <'a> Borrowable for Book<'a> {
    fn is_available_at(&self, year: u32) -> bool {
        if !self.exists_at(year) {
            return false;
        }

        // make sure its not borrowed at that time
        !self.checkout_history.get(&year).unwrap_or(&false)
    }

    fn return_at(&mut self, year: u32) -> Result<(), String> {
        if !self.exists_at(year) {
            return Err(format!("'{}' doesn't exist in {}!", self.title, year));
        }

        if self.is_available_at(year) {
            return Err(format!("'{}' wasn't checked out in {}", self.title, year));
        }

        self.checkout_history.insert(year, false);
        Ok(())
    }

    fn checkout_at(&mut self, year: u32) -> Result<(), String> {
        if !self.exists_at(year) {
            return Err(format!("'{}' doesn't exist in year {}", self.title, year));
        }

        if !self.is_available_at(year) {
            return Err(format!("'{}' is already checked out in year {}", self.title, year));
        }

        self.checkout_history.insert(year, true);
        Ok(())
    }
}

struct TimeLibrary<'a> {
    books: Vec<Book<'a>>,
}

impl<'a> TimeLibrary<'a> {
    fn new() -> Self {
        TimeLibrary { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book<'a>) {
        self.books.push(book);
    }

    fn available_books_in_year(&self, year: u32) -> Vec<&Book<'a>> {
        self.books
            .iter()
            .filter(|book| book.is_available_at(year))
            .collect()
    }
}

fn main() {
    let mut library = TimeLibrary::new();

    library.add_book(Book::new(
        "The Time Machine".to_string(),
        "H. G. Wells",
        1895,
        None // still exists
    ));

    library.add_book(Book::new(
        "Cardenio".to_string(),
        "William Shakespeare",
        1613,
        Some(1613)
    ));

    let book = &mut library.books[1];

    match book.checkout_at(1899) {
        Ok(_) => println!("Successfully checked out '{}' in 1899!", book.title),
        Err(e) => println!("Error: {}", e),
    }
}
