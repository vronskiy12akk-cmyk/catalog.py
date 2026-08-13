// catalog.cs — C# версия

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;

class Book {
    public int Id { get; set; }
    public string ISBN { get; set; }
    public string Title { get; set; }
    public string Author { get; set; }
    public int Year { get; set; }
    public string Publisher { get; set; }
}

class BookCatalog {
    private List<Book> books = new List<Book>();
    private const string DataFile = "books.json";

    public BookCatalog() {
        Load();
    }

    private void Load() {
        if (File.Exists(DataFile)) {
            try {
                string json = File.ReadAllText(DataFile);
                books = JsonSerializer.Deserialize<List<Book>>(json) ?? new List<Book>();
            } catch {
                books = new List<Book>();
            }
        }
    }

    private void Save() {
        string json = JsonSerializer.Serialize(books, new JsonSerializerOptions { WriteIndented = true });
        File.WriteAllText(DataFile, json);
    }

    private bool IsValidISBN(string isbn) {
        isbn = isbn.Replace("-", "").Replace(" ", "");
        if (isbn.Length == 10) return CheckISBN10(isbn);
        if (isbn.Length == 13) return CheckISBN13(isbn);
        return false;
    }

    private bool CheckISBN10(string isbn) {
        if (!System.Text.RegularExpressions.Regex.IsMatch(isbn, @"^\d{9}[\dX]$")) return false;
        int sum = 0;
        for (int i = 0; i < 9; i++) {
            sum += (i + 1) * (isbn[i] - '0');
        }
        char check = isbn[9];
        if (check == 'X') sum += 100;
        else sum += 10 * (check - '0');
        return sum % 11 == 0;
    }

    private bool CheckISBN13(string isbn) {
        if (!System.Text.RegularExpressions.Regex.IsMatch(isbn, @"^\d{13}$")) return false;
        int sum = 0;
        for (int i = 0; i < 13; i++) {
            int digit = isbn[i] - '0';
            sum += (i % 2 == 0) ? digit : 3 * digit;
        }
        return sum % 10 == 0;
    }

    public int AddBook(string isbn, string title, string author, int year, string publisher) {
        if (!IsValidISBN(isbn)) throw new Exception("Неверный ISBN");
        if (books.Any(b => b.ISBN == isbn)) throw new Exception("Книга с таким ISBN уже существует");
        int id = books.Count + 1;
        books.Add(new Book { Id = id, ISBN = isbn, Title = title, Author = author, Year = year, Publisher = publisher });
        Save();
        return id;
    }

    public void ListAll() {
        if (books.Count == 0) {
            Console.WriteLine("\u001B[33mКаталог пуст.\u001B[0m");
            return;
        }
        Console.WriteLine($"\u001B[36m{"ID",-4} {"ISBN",-15} {"Название",-30} {"Автор",-20} {"Год",-6} {"Издательство",-20}\u001B[0m");
        Console.WriteLine(new string('-', 100));
        foreach (var b in books) {
            string title = b.Title.Length > 30 ? b.Title.Substring(0, 30) : b.Title;
            string author = b.Author.Length > 20 ? b.Author.Substring(0, 20) : b.Author;
            string pub = b.Publisher.Length > 20 ? b.Publisher.Substring(0, 20) : b.Publisher;
            Console.WriteLine($"{b.Id,-4} {b.ISBN,-15} {title,-30} {author,-20} {b.Year,-6} {pub,-20}");
        }
    }

    public Book SearchByISBN(string isbn) {
        isbn = isbn.Replace("-", "").Replace(" ", "");
        return books.FirstOrDefault(b => b.ISBN == isbn);
    }

    public List<Book> SearchByText(string text) {
        text = text.ToLower();
        return books.Where(b => b.Title.ToLower().Contains(text) || b.Author.ToLower().Contains(text)).ToList();
    }

    public bool Delete(int id) {
        var book = books.FirstOrDefault(b => b.Id == id);
        if (book != null) {
            books.Remove(book);
            Save();
            return true;
        }
        return false;
    }

    public void Edit(int id, string field, string value) {
        var book = books.FirstOrDefault(b => b.Id == id);
        if (book == null) throw new Exception("Книга не найдена");
        switch (field.ToLower()) {
            case "isbn":
                if (!IsValidISBN(value)) throw new Exception("Неверный ISBN");
                if (books.Any(b => b.Id != id && b.ISBN == value)) throw new Exception("ISBN уже используется");
                book.ISBN = value;
                break;
            case "title": book.Title = value; break;
            case "author": book.Author = value; break;
            case "year": book.Year = int.Parse(value); break;
            case "publisher": book.Publisher = value; break;
            default: throw new Exception("Неизвестное поле");
        }
        Save();
    }

    public void Stats() {
        if (books.Count == 0) {
            Console.WriteLine("Нет данных.");
            return;
        }
        var authors = books.GroupBy(b => b.Author).ToDictionary(g => g.Key, g => g.Count());
        var years = books.GroupBy(b => b.Year).ToDictionary(g => g.Key, g => g.Count());
        Console.WriteLine("\u001B[36m📊 Статистика:\u001B[0m");
        Console.WriteLine($"  Всего книг: {books.Count}");
        Console.WriteLine("  По авторам:");
        foreach (var kv in authors.OrderByDescending(kv => kv.Value)) {
            Console.WriteLine($"    {kv.Key}: {kv.Value}");
        }
        Console.WriteLine("  По годам:");
        foreach (var kv in years.OrderBy(kv => kv.Key)) {
            Console.WriteLine($"    {kv.Key}: {kv.Value}");
        }
    }

    public static void Main() {
        var catalog = new BookCatalog();
        while (true) {
            Console.WriteLine("\n\u001B[36m📚 Book Catalog (ISBN) (C#)\u001B[0m");
            Console.WriteLine("1. Добавить книгу");
            Console.WriteLine("2. Показать все книги");
            Console.WriteLine("3. Поиск по ISBN");
            Console.WriteLine("4. Поиск по названию/автору");
            Console.WriteLine("5. Удалить книгу");
            Console.WriteLine("6. Редактировать книгу");
            Console.WriteLine("7. Статистика");
            Console.WriteLine("8. Выход");
            Console.Write("Выберите действие: ");
            string choice = Console.ReadLine().Trim();
            switch (choice) {
                case "1":
                    Console.Write("ISBN (10 или 13 цифр): ");
                    string isbn = Console.ReadLine().Trim();
                    Console.Write("Название: ");
                    string title = Console.ReadLine().Trim();
                    Console.Write("Автор: ");
                    string author = Console.ReadLine().Trim();
                    Console.Write("Год: ");
                    int year = int.Parse(Console.ReadLine().Trim());
                    Console.Write("Издательство: ");
                    string pub = Console.ReadLine().Trim();
                    try {
                        int id = catalog.AddBook(isbn, title, author, year, pub);
                        Console.WriteLine($"\u001B[32m✅ Книга добавлена (ID: {id})\u001B[0m");
                    } catch (Exception e) {
                        Console.WriteLine($"\u001B[31m❌ Ошибка: {e.Message}\u001B[0m");
                    }
                    break;
                case "2": catalog.ListAll(); break;
                case "3":
                    Console.Write("Введите ISBN: ");
                    string searchIsbn = Console.ReadLine().Trim();
                    var book = catalog.SearchByISBN(searchIsbn);
                    if (book != null) {
                        Console.WriteLine($"ID: {book.Id}\nISBN: {book.ISBN}\nНазвание: {book.Title}\nАвтор: {book.Author}\nГод: {book.Year}\nИздательство: {book.Publisher}");
                    } else {
                        Console.WriteLine("\u001B[33mКнига не найдена.\u001B[0m");
                    }
                    break;
                case "4":
                    Console.Write("Введите название или автора: ");
                    string text = Console.ReadLine().Trim();
                    var results = catalog.SearchByText(text);
                    if (results.Any()) {
                        foreach (var b in results) {
                            Console.WriteLine($"{b.Id}: {b.Title} | {b.Author} | {b.Year}");
                        }
                    } else {
                        Console.WriteLine("\u001B[33mНичего не найдено.\u001B[0m");
                    }
                    break;
                case "5":
                    catalog.ListAll();
                    Console.Write("Введите ID для удаления: ");
                    int delId = int.Parse(Console.ReadLine().Trim());
                    if (catalog.Delete(delId)) {
                        Console.WriteLine("\u001B[32m✅ Книга удалена.\u001B[0m");
                    } else {
                        Console.WriteLine("\u001B[31m❌ Книга не найдена.\u001B[0m");
                    }
                    break;
                case "6":
                    catalog.ListAll();
                    Console.Write("Введите ID для редактирования: ");
                    int editId = int.Parse(Console.ReadLine().Trim());
                    Console.Write("Какое поле редактировать (isbn, title, author, year, publisher): ");
                    string field = Console.ReadLine().Trim().ToLower();
                    Console.Write("Новое значение: ");
                    string value = Console.ReadLine().Trim();
                    try {
                        catalog.Edit(editId, field, value);
                        Console.WriteLine("\u001B[32m✅ Книга обновлена.\u001B[0m");
                    } catch (Exception e) {
                        Console.WriteLine($"\u001B[31m❌ Ошибка: {e.Message}\u001B[0m");
                    }
                    break;
                case "7": catalog.Stats(); break;
                case "8": Console.WriteLine("До свидания!"); return;
                default: Console.WriteLine("\u001B[31mНеверный выбор.\u001B[0m"); break;
            }
        }
    }
}
