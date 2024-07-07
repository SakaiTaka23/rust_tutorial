struct Book {
    title: String,
    author: String,
    genre: String,
}

struct BookIterator {
    properties: Vec<String>,
}

impl Iterator for BookIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.properties.is_empty() {
            Some(self.properties.remove(0))
        } else {
            None
        }
    }
}

impl IntoIterator for Book {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.title, self.author, self.genre].into_iter()
    }
}

fn main() {
    let book = Book {
        title: "title".to_string(),
        author: "author".to_string(),
        genre: "genre".to_string(),
    };

    let mut book_iterator = book.into_iter();

    for book_info in book_iterator {
        println!("{book_info}")
    }
}
