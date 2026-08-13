// catalog.rs — Rust версия

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Clone)]
struct Book {
    id: usize,
    isbn: String,
    title: String,
    author: String,
    year: u32,
    publisher: String,
}

struct Catalog {
    books: Vec<Book>,
    file: String,
}

impl Catalog {
    fn new(file: &str) -> Self {
        let mut c = Catalog { books: Vec::new(), file: file.to_string() };
        c.load();
        c
    }

    fn load(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.file) {
            if let Ok(books) = serde_json::from_str(&data) {
                self.books = books;
                return;
            }
        }
        self.books = Vec::new();
    }

    fn save(&self) {
        let data = serde_json::to_string_pretty(&self.books).unwrap();
        fs::write(&self.file, data).unwrap();
    }

    fn is_valid_isbn(isbn: &str) -> bool {
        let isbn = isbn.replace("-", "").replace(" ", "");
        if isbn.len() == 10 {
            Self::check_isbn10(&isbn)
        } else if isbn.len() == 13 {
            Self::check_isbn13(&isbn)
        } else {
            false
        }
    }

    fn check_isbn10(isbn: &str) -> bool {
        if !isbn.chars().take(9).all(|c| c.is_ascii_digit()) {
            return false;
        }
        let sum: u32 = (0..9).map(|i| (i + 1) as u32 * (isbn.chars().nth(i).unwrap() as u32 - '0' as u32)).sum();
        let check = isbn.chars().nth(9).unwrap();
        let total = if check == 'X' {
            sum + 100
        } else if check.is_ascii_digit() {
            sum + 10 * (check as u32 - '0' as u32)
        } else {
            return false;
        };
        total % 11 == 0
    }

    fn check_isbn13(isbn: &str) -> bool {
        if !isbn.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let sum: u32 = isbn.chars().enumerate().map(|(i, c)| {
            let digit = c as u32 - '0' as u32;
            if i % 2 == 0 { digit } else { 3 * digit }
        }).sum();
        sum % 10 == 0
    }

    fn add_book(&mut self, isbn: String, title: String, author: String, year: u32, publisher: String) -> Result<usize, String> {
        if !Self::is_valid_isbn(&isbn) {
            return Err("Неверный ISBN".to_string());
        }
        if self.books.iter().any(|b| b.isbn == isbn) {
            return Err("Книга с таким ISBN уже существует".to_string());
        }
        let id = self.books.len() + 1;
        self.books.push(Book { id, isbn, title, author, year, publisher });
        self.save();
        Ok(id)
    }

    fn list_all(&self) {
        if self.books.is_empty() {
            println!("\x1b[33mКаталог пуст.\x1b[0m");
            return;
        }
        println!("\x1b[36m{:<4} {:<15} {:<30} {:<20} {:<6} {:<20}\x1b[0m", "ID", "ISBN", "Название", "Автор", "Год", "Издательство");
        println!("{}", "-".repeat(100));
        for b in &self.books {
            let title = if b.title.len() > 30 { &b.title[..30] } else { &b.title };
            let author = if b.author.len() > 20 { &b.author[..20] } else { &b.author };
            let pub = if b.publisher.len() > 20 { &b.publisher[..20] } else { &b.publisher };
            println!("{:<4} {:<15} {:<30} {:<20} {:<6} {:<20}", b.id, b.isbn, title, author, b.year, pub);
        }
    }

    fn search_by_isbn(&self, isbn: &str) -> Option<&Book> {
        let isbn = isbn.replace("-", "").replace(" ", "");
        self.books.iter().find(|b| b.isbn == isbn)
    }

    fn search_by_text(&self, text: &str) -> Vec<&Book> {
        let text = text.to_lowercase();
        self.books.iter().filter(|b| {
            b.title.to_lowercase().contains(&text) || b.author.to_lowercase().contains(&text)
        }).collect()
    }

    fn delete(&mut self, id: usize) -> bool {
        let pos = self.books.iter().position(|b| b.id == id);
        if let Some(idx) = pos {
            self.books.remove(idx);
            self.save();
            true
        } else {
            false
        }
    }

    fn edit(&mut self, id: usize, field: &str, value: &str) -> Result<(), String> {
        for b in &mut self.books {
            if b.id == id {
                match field {
                    "isbn" => {
                        if !Self::is_valid_isbn(value) {
                            return Err("Неверный ISBN".to_string());
                        }
                        if self.books.iter().any(|other| other.id != id && other.isbn == value) {
                            return Err("ISBN уже используется".to_string());
                        }
                        b.isbn = value.to_string();
                    }
                    "title" => b.title = value.to_string(),
                    "author" => b.author = value.to_string(),
                    "year" => {
                        let y = value.parse::<u32>().map_err(|_| "Неверный год")?;
                        b.year = y;
                    }
                    "publisher" => b.publisher = value.to_string(),
                    _ => return Err("Неизвестное поле".to_string()),
                }
                self.save();
                return Ok(());
            }
        }
        Err("Книга не найдена".to_string())
    }

    fn stats(&self) {
        if self.books.is_empty() {
            println!("Нет данных.");
            return;
        }
        let mut authors = HashMap::new();
        let mut years = HashMap::new();
        for b in &self.books {
            *authors.entry(&b.author).or_insert(0) += 1;
            *years.entry(b.year).or_insert(0) += 1;
        }
        println!("\x1b[36m📊 Статистика:\x1b[0m");
        println!("  Всего книг: {}", self.books.len());
        println!("  По авторам:");
        let mut authors_vec: Vec<_> = authors.iter().collect();
        authors_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (a, c) in authors_vec {
            println!("    {}: {}", a, c);
        }
        println!("  По годам:");
        let mut years_vec: Vec<_> = years.iter().collect();
        years_vec.sort_by(|a, b| a.0.cmp(b.0));
        for (y, c) in years_vec {
            println!("    {}: {}", y, c);
        }
    }
}

fn main() {
    let mut catalog = Catalog::new("books.json");
    loop {
        println!("\n\x1b[36m📚 Book Catalog (ISBN) (Rust)\x1b[0m");
        println!("1. Добавить книгу");
        println!("2. Показать все книги");
        println!("3. Поиск по ISBN");
        println!("4. Поиск по названию/автору");
        println!("5. Удалить книгу");
        println!("6. Редактировать книгу");
        println!("7. Статистика");
        println!("8. Выход");
        print!("Выберите действие: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                print!("ISBN (10 или 13 цифр): ");
                io::stdout().flush().unwrap();
                let mut isbn = String::new();
                io::stdin().read_line(&mut isbn).unwrap();
                let isbn = isbn.trim().to_string();
                print!("Название: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string();
                print!("Автор: ");
                io::stdout().flush().unwrap();
                let mut author = String::new();
                io::stdin().read_line(&mut author).unwrap();
                let author = author.trim().to_string();
                print!("Год: ");
                io::stdout().flush().unwrap();
                let mut year_str = String::new();
                io::stdin().read_line(&mut year_str).unwrap();
                let year: u32 = year_str.trim().parse().unwrap();
                print!("Издательство: ");
                io::stdout().flush().unwrap();
                let mut pub = String::new();
                io::stdin().read_line(&mut pub).unwrap();
                let pub = pub.trim().to_string();
                match catalog.add_book(isbn, title, author, year, pub) {
                    Ok(id) => println!("\x1b[32m✅ Книга добавлена (ID: {})\x1b[0m", id),
                    Err(e) => println!("\x1b[31m❌ Ошибка: {}\x1b[0m", e),
                }
            }
            "2" => catalog.list_all(),
            "3" => {
                print!("Введите ISBN: ");
                io::stdout().flush().unwrap();
                let mut isbn = String::new();
                io::stdin().read_line(&mut isbn).unwrap();
                let isbn = isbn.trim();
                if let Some(book) = catalog.search_by_isbn(isbn) {
                    println!("ID: {}\nISBN: {}\nНазвание: {}\nАвтор: {}\nГод: {}\nИздательство: {}",
                        book.id, book.isbn, book.title, book.author, book.year, book.publisher);
                } else {
                    println!("\x1b[33mКнига не найдена.\x1b[0m");
                }
            }
            "4" => {
                print!("Введите название или автора: ");
                io::stdout().flush().unwrap();
                let mut text = String::new();
                io::stdin().read_line(&mut text).unwrap();
                let text = text.trim();
                let results = catalog.search_by_text(text);
                if results.is_empty() {
                    println!("\x1b[33mНичего не найдено.\x1b[0m");
                } else {
                    for b in results {
                        println!("{}: {} | {} | {}", b.id, b.title, b.author, b.year);
                    }
                }
            }
            "5" => {
                catalog.list_all();
                print!("Введите ID для удаления: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: usize = id_str.trim().parse().unwrap();
                if catalog.delete(id) {
                    println!("\x1b[32m✅ Книга удалена.\x1b[0m");
                } else {
                    println!("\x1b[31m❌ Книга не найдена.\x1b[0m");
                }
            }
            "6" => {
                catalog.list_all();
                print!("Введите ID для редактирования: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: usize = id_str.trim().parse().unwrap();
                print!("Какое поле редактировать (isbn, title, author, year, publisher): ");
                io::stdout().flush().unwrap();
                let mut field = String::new();
                io::stdin().read_line(&mut field).unwrap();
                let field = field.trim().to_lowercase();
                print!("Новое значение: ");
                io::stdout().flush().unwrap();
                let mut value = String::new();
                io::stdin().read_line(&mut value).unwrap();
                let value = value.trim();
                match catalog.edit(id, &field, value) {
                    Ok(()) => println!("\x1b[32m✅ Книга обновлена.\x1b[0m"),
                    Err(e) => println!("\x1b[31m❌ Ошибка: {}\x1b[0m", e),
                }
            }
            "7" => catalog.stats(),
            "8" => {
                println!("До свидания!");
                break;
            }
            _ => println!("\x1b[31mНеверный выбор.\x1b[0m"),
        }
    }
}
