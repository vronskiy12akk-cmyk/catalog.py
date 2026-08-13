// catalog.js — JavaScript версия

const fs = require('fs');
const readline = require('readline');

const DATA_FILE = 'books.json';

class Book {
    constructor(id, isbn, title, author, year, publisher) {
        this.id = id;
        this.isbn = isbn;
        this.title = title;
        this.author = author;
        this.year = year;
        this.publisher = publisher;
    }
}

class BookCatalog {
    constructor() {
        this.books = [];
        this.load();
    }

    load() {
        if (fs.existsSync(DATA_FILE)) {
            try {
                const data = JSON.parse(fs.readFileSync(DATA_FILE, 'utf8'));
                this.books = data.map(b => new Book(b.id, b.isbn, b.title, b.author, b.year, b.publisher));
            } catch {
                this.books = [];
            }
        }
    }

    save() {
        fs.writeFileSync(DATA_FILE, JSON.stringify(this.books, null, 2));
    }

    isValidISBN(isbn) {
        isbn = isbn.replace(/-/g, '').replace(/ /g, '');
        if (isbn.length === 10) return this._checkISBN10(isbn);
        if (isbn.length === 13) return this._checkISBN13(isbn);
        return false;
    }

    _checkISBN10(isbn) {
        if (!/^\d{9}[\dX]$/.test(isbn)) return false;
        let sum = 0;
        for (let i = 0; i < 9; i++) {
            sum += (i + 1) * parseInt(isbn[i]);
        }
        const check = isbn[9];
        if (check === 'X') sum += 100;
        else sum += 10 * parseInt(check);
        return sum % 11 === 0;
    }

    _checkISBN13(isbn) {
        if (!/^\d{13}$/.test(isbn)) return false;
        let sum = 0;
        for (let i = 0; i < 13; i++) {
            const digit = parseInt(isbn[i]);
            sum += (i % 2 === 0) ? digit : 3 * digit;
        }
        return sum % 10 === 0;
    }

    addBook(isbn, title, author, year, publisher) {
        if (!this.isValidISBN(isbn)) throw new Error('Неверный ISBN');
        if (this.books.some(b => b.isbn === isbn)) throw new Error('Книга с таким ISBN уже существует');
        const id = this.books.length + 1;
        const book = new Book(id, isbn, title, author, year, publisher);
        this.books.push(book);
        this.save();
        return id;
    }

    listAll() {
        if (this.books.length === 0) {
            console.log('\x1b[33mКаталог пуст.\x1b[0m');
            return;
        }
        console.log('\x1b[36m' + 'ID'.padEnd(4) + 'ISBN'.padEnd(15) + 'Название'.padEnd(30) + 'Автор'.padEnd(20) + 'Год'.padEnd(6) + 'Издательство'.padEnd(20) + '\x1b[0m');
        console.log('-'.repeat(100));
        for (const b of this.books) {
            const title = b.title.length > 30 ? b.title.slice(0, 30) : b.title;
            const author = b.author.length > 20 ? b.author.slice(0, 20) : b.author;
            const pub = b.publisher.length > 20 ? b.publisher.slice(0, 20) : b.publisher;
            console.log(`${String(b.id).padEnd(4)} ${b.isbn.padEnd(15)} ${title.padEnd(30)} ${author.padEnd(20)} ${String(b.year).padEnd(6)} ${pub.padEnd(20)}`);
        }
    }

    searchByISBN(isbn) {
        isbn = isbn.replace(/-/g, '').replace(/ /g, '');
        return this.books.find(b => b.isbn === isbn) || null;
    }

    searchByText(text) {
        text = text.toLowerCase();
        return this.books.filter(b => b.title.toLowerCase().includes(text) || b.author.toLowerCase().includes(text));
    }

    delete(id) {
        const index = this.books.findIndex(b => b.id === id);
        if (index !== -1) {
            this.books.splice(index, 1);
            this.save();
            return true;
        }
        return false;
    }

    edit(id, field, value) {
        const book = this.books.find(b => b.id === id);
        if (!book) throw new Error('Книга не найдена');
        switch (field) {
            case 'isbn':
                if (!this.isValidISBN(value)) throw new Error('Неверный ISBN');
                if (this.books.some(b => b.id !== id && b.isbn === value)) throw new Error('ISBN уже используется');
                book.isbn = value;
                break;
            case 'title': book.title = value; break;
            case 'author': book.author = value; break;
            case 'year': book.year = parseInt(value); break;
            case 'publisher': book.publisher = value; break;
            default: throw new Error('Неизвестное поле');
        }
        this.save();
        return true;
    }

    stats() {
        if (this.books.length === 0) {
            console.log('Нет данных.');
            return;
        }
        const total = this.books.length;
        const authors = {};
        const years = {};
        for (const b of this.books) {
            authors[b.author] = (authors[b.author] || 0) + 1;
            years[b.year] = (years[b.year] || 0) + 1;
        }
        console.log('\x1b[36m📊 Статистика:\x1b[0m');
        console.log(`  Всего книг: ${total}`);
        console.log('  По авторам:');
        for (const [a, c] of Object.entries(authors).sort((a, b) => b[1] - a[1])) {
            console.log(`    ${a}: ${c}`);
        }
        console.log('  По годам:');
        for (const [y, c] of Object.entries(years).sort()) {
            console.log(`    ${y}: ${c}`);
        }
    }
}

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

const catalog = new BookCatalog();

function ask(question) {
    return new Promise(resolve => rl.question(question, resolve));
}

async function main() {
    while (true) {
        console.log('\x1b[36m\n📚 Book Catalog (ISBN) (JavaScript)\x1b[0m');
        console.log('1. Добавить книгу');
        console.log('2. Показать все книги');
        console.log('3. Поиск по ISBN');
        console.log('4. Поиск по названию/автору');
        console.log('5. Удалить книгу');
        console.log('6. Редактировать книгу');
        console.log('7. Статистика');
        console.log('8. Выход');
        const choice = await ask('Выберите действие: ');
        switch (choice.trim()) {
            case '1': {
                const isbn = await ask('ISBN (10 или 13 цифр): ');
                const title = await ask('Название: ');
                const author = await ask('Автор: ');
                const year = parseInt(await ask('Год: '));
                const publisher = await ask('Издательство: ');
                try {
                    const id = catalog.addBook(isbn, title, author, year, publisher);
                    console.log(`\x1b[32m✅ Книга добавлена (ID: ${id})\x1b[0m`);
                } catch (e) {
                    console.log(`\x1b[31m❌ Ошибка: ${e.message}\x1b[0m`);
                }
                break;
            }
            case '2': catalog.listAll(); break;
            case '3': {
                const isbn = await ask('Введите ISBN: ');
                const book = catalog.searchByISBN(isbn);
                if (book) {
                    console.log(`ID: ${book.id}\nISBN: ${book.isbn}\nНазвание: ${book.title}\nАвтор: ${book.author}\nГод: ${book.year}\nИздательство: ${book.publisher}`);
                } else {
                    console.log('\x1b[33mКнига не найдена.\x1b[0m');
                }
                break;
            }
            case '4': {
                const text = await ask('Введите название или автора: ');
                const results = catalog.searchByText(text);
                if (results.length) {
                    for (const b of results) {
                        console.log(`${b.id}: ${b.title} | ${b.author} | ${b.year}`);
                    }
                } else {
                    console.log('\x1b[33mНичего не найдено.\x1b[0m');
                }
                break;
            }
            case '5': {
                catalog.listAll();
                const id = parseInt(await ask('Введите ID для удаления: '));
                if (catalog.delete(id)) {
                    console.log('\x1b[32m✅ Книга удалена.\x1b[0m');
                } else {
                    console.log('\x1b[31m❌ Книга не найдена.\x1b[0m');
                }
                break;
            }
            case '6': {
                catalog.listAll();
                const id = parseInt(await ask('Введите ID для редактирования: '));
                const field = await ask('Какое поле редактировать (isbn, title, author, year, publisher): ');
                const value = await ask('Новое значение: ');
                try {
                    catalog.edit(id, field.trim(), value.trim());
                    console.log('\x1b[32m✅ Книга обновлена.\x1b[0m');
                } catch (e) {
                    console.log(`\x1b[31m❌ Ошибка: ${e.message}\x1b[0m`);
                }
                break;
            }
            case '7': catalog.stats(); break;
            case '8':
                console.log('До свидания!');
                rl.close();
                return;
            default:
                console.log('\x1b[31mНеверный выбор.\x1b[0m');
        }
    }
}

main().catch(console.error);
