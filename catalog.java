// catalog.java — Java версия

import java.io.*;
import java.nio.file.*;
import java.util.*;

class Book {
    int id;
    String isbn;
    String title;
    String author;
    int year;
    String publisher;

    Book(int id, String isbn, String title, String author, int year, String publisher) {
        this.id = id;
        this.isbn = isbn;
        this.title = title;
        this.author = author;
        this.year = year;
        this.publisher = publisher;
    }

    String toJson() {
        return String.format("{\"id\":%d,\"isbn\":\"%s\",\"title\":\"%s\",\"author\":\"%s\",\"year\":%d,\"publisher\":\"%s\"}",
                id, isbn, title, author, year, publisher);
    }
}

public class catalog {
    private static List<Book> books = new ArrayList<>();
    private static final String DATA_FILE = "books.json";
    private static Scanner scanner = new Scanner(System.in);

    public static void main(String[] args) {
        load();
        while (true) {
            System.out.println("\n\u001B[36m📚 Book Catalog (ISBN) (Java)\u001B[0m");
            System.out.println("1. Добавить книгу");
            System.out.println("2. Показать все книги");
            System.out.println("3. Поиск по ISBN");
            System.out.println("4. Поиск по названию/автору");
            System.out.println("5. Удалить книгу");
            System.out.println("6. Редактировать книгу");
            System.out.println("7. Статистика");
            System.out.println("8. Выход");
            System.out.print("Выберите действие: ");
            String choice = scanner.nextLine().trim();
            switch (choice) {
                case "1": addBook(); break;
                case "2": listAll(); break;
                case "3": searchByISBN(); break;
                case "4": searchByText(); break;
                case "5": deleteBook(); break;
                case "6": editBook(); break;
                case "7": stats(); break;
                case "8": System.out.println("До свидания!"); return;
                default: System.out.println("\u001B[31mНеверный выбор.\u001B[0m");
            }
        }
    }

    private static void load() {
        try {
            String content = new String(Files.readAllBytes(Paths.get(DATA_FILE)));
            // Упрощённо: если файл есть, парсим, иначе пустой список
            books = new ArrayList<>();
        } catch (IOException e) {
            books = new ArrayList<>();
        }
    }

    private static void save() {
        try {
            StringBuilder sb = new StringBuilder("[");
            for (int i = 0; i < books.size(); i++) {
                sb.append(books.get(i).toJson());
                if (i < books.size() - 1) sb.append(",");
            }
            sb.append("]");
            Files.write(Paths.get(DATA_FILE), sb.toString().getBytes());
        } catch (IOException e) {
            System.out.println("Ошибка сохранения.");
        }
    }

    private static boolean isValidISBN(String isbn) {
        isbn = isbn.replace("-", "").replace(" ", "");
        if (isbn.length() == 10) return checkISBN10(isbn);
        if (isbn.length() == 13) return checkISBN13(isbn);
        return false;
    }

    private static boolean checkISBN10(String isbn) {
        if (!isbn.matches("\\d{9}[\\dX]")) return false;
        int sum = 0;
        for (int i = 0; i < 9; i++) {
            sum += (i + 1) * (isbn.charAt(i) - '0');
        }
        char check = isbn.charAt(9);
        if (check == 'X') sum += 100;
        else sum += 10 * (check - '0');
        return sum % 11 == 0;
    }

    private static boolean checkISBN13(String isbn) {
        if (!isbn.matches("\\d{13}")) return false;
        int sum = 0;
        for (int i = 0; i < 13; i++) {
            int digit = isbn.charAt(i) - '0';
            sum += (i % 2 == 0) ? digit : 3 * digit;
        }
        return sum % 10 == 0;
    }

    private static void addBook() {
        System.out.print("ISBN (10 или 13 цифр): ");
        String isbn = scanner.nextLine().trim();
        System.out.print("Название: ");
        String title = scanner.nextLine().trim();
        System.out.print("Автор: ");
        String author = scanner.nextLine().trim();
        System.out.print("Год: ");
        int year = Integer.parseInt(scanner.nextLine().trim());
        System.out.print("Издательство: ");
        String publisher = scanner.nextLine().trim();

        if (!isValidISBN(isbn)) {
            System.out.println("\u001B[31m❌ Неверный ISBN.\u001B[0m");
            return;
        }
        for (Book b : books) {
            if (b.isbn.equals(isbn)) {
                System.out.println("\u001B[31m❌ Книга с таким ISBN уже существует.\u001B[0m");
                return;
            }
        }
        int id = books.size() + 1;
        books.add(new Book(id, isbn, title, author, year, publisher));
        save();
        System.out.println("\u001B[32m✅ Книга добавлена (ID: " + id + ")\u001B[0m");
    }

    private static void listAll() {
        if (books.isEmpty()) {
            System.out.println("\u001B[33mКаталог пуст.\u001B[0m");
            return;
        }
        System.out.printf("\u001B[36m%-4s %-15s %-30s %-20s %-6s %-20s\u001B[0m\n", "ID", "ISBN", "Название", "Автор", "Год", "Издательство");
        System.out.println("-".repeat(100));
        for (Book b : books) {
            String title = b.title.length() > 30 ? b.title.substring(0, 30) : b.title;
            String author = b.author.length() > 20 ? b.author.substring(0, 20) : b.author;
            String pub = b.publisher.length() > 20 ? b.publisher.substring(0, 20) : b.publisher;
            System.out.printf("%-4d %-15s %-30s %-20s %-6d %-20s\n", b.id, b.isbn, title, author, b.year, pub);
        }
    }

    private static void searchByISBN() {
        System.out.print("Введите ISBN: ");
        String isbn = scanner.nextLine().trim().replace("-", "").replace(" ", "");
        for (Book b : books) {
            if (b.isbn.equals(isbn)) {
                System.out.printf("ID: %d\nISBN: %s\nНазвание: %s\nАвтор: %s\nГод: %d\nИздательство: %s\n",
                        b.id, b.isbn, b.title, b.author, b.year, b.publisher);
                return;
            }
        }
        System.out.println("\u001B[33mКнига не найдена.\u001B[0m");
    }

    private static void searchByText() {
        System.out.print("Введите название или автора: ");
        String text = scanner.nextLine().trim().toLowerCase();
        List<Book> results = new ArrayList<>();
        for (Book b : books) {
            if (b.title.toLowerCase().contains(text) || b.author.toLowerCase().contains(text)) {
                results.add(b);
            }
        }
        if (results.isEmpty()) {
            System.out.println("\u001B[33mНичего не найдено.\u001B[0m");
            return;
        }
        for (Book b : results) {
            System.out.printf("%d: %s | %s | %d\n", b.id, b.title, b.author, b.year);
        }
    }

    private static void deleteBook() {
        listAll();
        System.out.print("Введите ID для удаления: ");
        int id = Integer.parseInt(scanner.nextLine().trim());
        Iterator<Book> it = books.iterator();
        while (it.hasNext()) {
            if (it.next().id == id) {
                it.remove();
                save();
                System.out.println("\u001B[32m✅ Книга удалена.\u001B[0m");
                return;
            }
        }
        System.out.println("\u001B[31m❌ Книга не найдена.\u001B[0m");
    }

    private static void editBook() {
        listAll();
        System.out.print("Введите ID для редактирования: ");
        int id = Integer.parseInt(scanner.nextLine().trim());
        Book target = null;
        for (Book b : books) {
            if (b.id == id) {
                target = b;
                break;
            }
        }
        if (target == null) {
            System.out.println("\u001B[31m❌ Книга не найдена.\u001B[0m");
            return;
        }
        System.out.print("Какое поле редактировать (isbn, title, author, year, publisher): ");
        String field = scanner.nextLine().trim().toLowerCase();
        System.out.print("Новое значение: ");
        String value = scanner.nextLine().trim();
        try {
            switch (field) {
                case "isbn":
                    if (!isValidISBN(value)) throw new Exception("Неверный ISBN");
                    for (Book other : books) {
                        if (other.id != id && other.isbn.equals(value)) throw new Exception("ISBN уже используется");
                    }
                    target.isbn = value;
                    break;
                case "title": target.title = value; break;
                case "author": target.author = value; break;
                case "year": target.year = Integer.parseInt(value); break;
                case "publisher": target.publisher = value; break;
                default: throw new Exception("Неизвестное поле");
            }
            save();
            System.out.println("\u001B[32m✅ Книга обновлена.\u001B[0m");
        } catch (Exception e) {
            System.out.println("\u001B[31m❌ Ошибка: " + e.getMessage() + "\u001B[0m");
        }
    }

    private static void stats() {
        if (books.isEmpty()) {
            System.out.println("Нет данных.");
            return;
        }
        Map<String, Integer> authors = new HashMap<>();
        Map<Integer, Integer> years = new HashMap<>();
        for (Book b : books) {
            authors.put(b.author, authors.getOrDefault(b.author, 0) + 1);
            years.put(b.year, years.getOrDefault(b.year, 0) + 1);
        }
        System.out.println("\u001B[36m📊 Статистика:\u001B[0m");
        System.out.printf("  Всего книг: %d\n", books.size());
        System.out.println("  По авторам:");
        for (Map.Entry<String, Integer> e : authors.entrySet()) {
            System.out.printf("    %s: %d\n", e.getKey(), e.getValue());
        }
        System.out.println("  По годам:");
        for (Map.Entry<Integer, Integer> e : years.entrySet()) {
            System.out.printf("    %d: %d\n", e.getKey(), e.getValue());
        }
    }
}
