pub struct Library {
    books: Vec<Book>,
}

pub struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    pub fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    pub fn new() -> Library {
        Library { books: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.books.len()
    }

    pub fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn print_books(&self) {
        self.books.iter().for_each(|b| println!("{b}"));
    }

    pub fn oldest_book(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|&b| b.year)
    }
}
