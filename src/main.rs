enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

fn main() {

    let book = Book {
        title: String::from("Rust Programming Language"),
        author: String::from("Steve Klabnik"),
        page_count: 432,
    };

    let magazine = Magazine {
        title: String::from("Bilim Teknik"),
        issue: 123,
        topic: String::from("Science and Technology"),
    };

    let publications: Vec<Publication> = vec![
        Publication::Book(book),
        Publication::Magazine(magazine),
    ];

    for publ in publications.iter() {
        match publ {
            Publication::Book(book) => {
                println!(
                    "Kitap: {} yazar: {}, {} sayfa",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(magazine) => {
                println!(
                    "Dergi: {} - Sayı: {}, Konu: {}",
                    magazine.title, magazine.issue, magazine.topic
                );
            }
        }
    }
}