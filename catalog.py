

### 1. `catalog.py` (Python)

```python
# catalog.py — Python версия

import json
import os
import re
from datetime import datetime
from colorama import init, Fore, Style

init(autoreset=True)
DATA_FILE = "books.json"

class Book:
    def __init__(self, id, isbn, title, author, year, publisher):
        self.id = id
        self.isbn = isbn
        self.title = title
        self.author = author
        self.year = year
        self.publisher = publisher

    def to_dict(self):
        return {"id": self.id, "isbn": self.isbn, "title": self.title,
                "author": self.author, "year": self.year, "publisher": self.publisher}

    @classmethod
    def from_dict(cls, data):
        return cls(data["id"], data["isbn"], data["title"], data["author"],
                   data["year"], data["publisher"])

class BookCatalog:
    def __init__(self):
        self.books = []
        self.load()

    def load(self):
        if os.path.exists(DATA_FILE):
            try:
                with open(DATA_FILE, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                    self.books = [Book.from_dict(b) for b in data]
            except:
                self.books = []

    def save(self):
        with open(DATA_FILE, 'w', encoding='utf-8') as f:
            json.dump([b.to_dict() for b in self.books], f, indent=2, ensure_ascii=False)

    def is_valid_isbn(self, isbn):
        """Проверка ISBN-10 и ISBN-13."""
        isbn = isbn.replace('-', '').replace(' ', '')
        if len(isbn) == 10:
            return self._check_isbn10(isbn)
        elif len(isbn) == 13:
            return self._check_isbn13(isbn)
        return False

    def _check_isbn10(self, isbn):
        if not isbn[:-1].isdigit():
            return False
        total = 0
        for i in range(9):
            total += (i + 1) * int(isbn[i])
        check = isbn[9]
        if check == 'X':
            total += 10 * 10
        elif check.isdigit():
            total += 10 * int(check)
        else:
            return False
        return total % 11 == 0

    def _check_isbn13(self, isbn):
        if not isbn.isdigit():
            return False
        total = 0
        for i in range(13):
            if i % 2 == 0:
                total += int(isbn[i])
            else:
                total += 3 * int(isbn[i])
        return total % 10 == 0

    def add_book(self, isbn, title, author, year, publisher):
        if not self.is_valid_isbn(isbn):
            raise ValueError("Неверный ISBN")
        # Проверка на уникальность ISBN
        for b in self.books:
            if b.isbn == isbn:
                raise ValueError("Книга с таким ISBN уже существует")
        id = len(self.books) + 1
        book = Book(id, isbn, title, author, year, publisher)
        self.books.append(book)
        self.save()
        return id

    def list_all(self):
        if not self.books:
            print(Fore.YELLOW + "Каталог пуст.")
            return
        print(Fore.CYAN + f"{'ID':<4} {'ISBN':<15} {'Название':<30} {'Автор':<20} {'Год':<6} {'Издательство':<20}")
        print("-" * 100)
        for b in self.books:
            print(f"{b.id:<4} {b.isbn:<15} {b.title[:30]:<30} {b.author[:20]:<20} {b.year:<6} {b.publisher[:20]:<20}")

    def search_by_isbn(self, isbn):
        isbn = isbn.replace('-', '').replace(' ', '')
        for b in self.books:
            if b.isbn == isbn:
                return b
        return None

    def search_by_text(self, text):
        text = text.lower()
        results = [b for b in self.books if text in b.title.lower() or text in b.author.lower()]
        return results

    def delete(self, id):
        for i, b in enumerate(self.books):
            if b.id == id:
                del self.books[i]
                self.save()
                return True
        return False

    def edit(self, id, field, value):
        for b in self.books:
            if b.id == id:
                if field == "isbn":
                    if not self.is_valid_isbn(value):
                        raise ValueError("Неверный ISBN")
                    # Проверка уникальности
                    for other in self.books:
                        if other.id != id and other.isbn == value:
                            raise ValueError("ISBN уже используется")
                    b.isbn = value
                elif field == "title":
                    b.title = value
                elif field == "author":
                    b.author = value
                elif field == "year":
                    b.year = int(value)
                elif field == "publisher":
                    b.publisher = value
                else:
                    return False
                self.save()
                return True
        return False

    def stats(self):
        if not self.books:
            print("Нет данных.")
            return
        total = len(self.books)
        authors = {}
        years = {}
        for b in self.books:
            authors[b.author] = authors.get(b.author, 0) + 1
            years[b.year] = years.get(b.year, 0) + 1
        print(Fore.CYAN + "📊 Статистика:")
        print(f"  Всего книг: {total}")
        print("  По авторам:")
        for a, c in sorted(authors.items(), key=lambda x: -x[1]):
            print(f"    {a}: {c}")
        print("  По годам:")
        for y, c in sorted(years.items()):
            print(f"    {y}: {c}")

def main():
    catalog = BookCatalog()
    while True:
        print(Fore.CYAN + "\n📚 Book Catalog (ISBN) (Python)")
        print("1. Добавить книгу")
        print("2. Показать все книги")
        print("3. Поиск по ISBN")
        print("4. Поиск по названию/автору")
        print("5. Удалить книгу")
        print("6. Редактировать книгу")
        print("7. Статистика")
        print("8. Выход")
        choice = input("Выберите действие: ").strip()
        if choice == "1":
            isbn = input("ISBN (10 или 13 цифр): ").strip()
            title = input("Название: ").strip()
            author = input("Автор: ").strip()
            year = int(input("Год: ").strip())
            publisher = input("Издательство: ").strip()
            try:
                id = catalog.add_book(isbn, title, author, year, publisher)
                print(Fore.GREEN + f"✅ Книга добавлена (ID: {id})")
            except Exception as e:
                print(Fore.RED + f"❌ Ошибка: {e}")
        elif choice == "2":
            catalog.list_all()
        elif choice == "3":
            isbn = input("Введите ISBN: ").strip()
            book = catalog.search_by_isbn(isbn)
            if book:
                print(f"ID: {book.id}\nISBN: {book.isbn}\nНазвание: {book.title}\nАвтор: {book.author}\nГод: {book.year}\nИздательство: {book.publisher}")
            else:
                print(Fore.YELLOW + "Книга не найдена.")
        elif choice == "4":
            text = input("Введите название или автора: ").strip()
            results = catalog.search_by_text(text)
            if results:
                for b in results:
                    print(f"{b.id}: {b.title} | {b.author} | {b.year}")
            else:
                print(Fore.YELLOW + "Ничего не найдено.")
        elif choice == "5":
            catalog.list_all()
            id = int(input("Введите ID для удаления: ").strip())
            if catalog.delete(id):
                print(Fore.GREEN + "✅ Книга удалена.")
            else:
                print(Fore.RED + "❌ Книга не найдена.")
        elif choice == "6":
            catalog.list_all()
            id = int(input("Введите ID для редактирования: ").strip())
            field = input("Какое поле редактировать (isbn, title, author, year, publisher): ").strip().lower()
            value = input("Новое значение: ").strip()
            try:
                if catalog.edit(id, field, value):
                    print(Fore.GREEN + "✅ Книга обновлена.")
                else:
                    print(Fore.RED + "❌ Не удалось обновить.")
            except Exception as e:
                print(Fore.RED + f"❌ Ошибка: {e}")
        elif choice == "7":
            catalog.stats()
        elif choice == "8":
            print("До свидания!")
            break
        else:
            print(Fore.RED + "Неверный выбор.")

if __name__ == "__main__":
    main()
