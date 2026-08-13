// catalog.go — Go версия

package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Book struct {
	ID        int    `json:"id"`
	ISBN      string `json:"isbn"`
	Title     string `json:"title"`
	Author    string `json:"author"`
	Year      int    `json:"year"`
	Publisher string `json:"publisher"`
}

type Catalog struct {
	Books []Book `json:"books"`
	file  string
}

func NewCatalog(file string) *Catalog {
	c := &Catalog{file: file}
	c.load()
	return c
}

func (c *Catalog) load() {
	data, err := os.ReadFile(c.file)
	if err != nil {
		c.Books = []Book{}
		return
	}
	json.Unmarshal(data, &c.Books)
}

func (c *Catalog) save() {
	data, _ := json.MarshalIndent(c.Books, "", "  ")
	os.WriteFile(c.file, data, 0644)
}

func (c *Catalog) isValidISBN(isbn string) bool {
	isbn = strings.ReplaceAll(isbn, "-", "")
	isbn = strings.ReplaceAll(isbn, " ", "")
	if len(isbn) == 10 {
		return c.checkISBN10(isbn)
	} else if len(isbn) == 13 {
		return c.checkISBN13(isbn)
	}
	return false
}

func (c *Catalog) checkISBN10(isbn string) bool {
	if !isISBNDigit(isbn[:9]) {
		return false
	}
	sum := 0
	for i := 0; i < 9; i++ {
		sum += (i + 1) * int(isbn[i]-'0')
	}
	check := isbn[9]
	if check == 'X' {
		sum += 10 * 10
	} else if check >= '0' && check <= '9' {
		sum += 10 * int(check-'0')
	} else {
		return false
	}
	return sum%11 == 0
}

func (c *Catalog) checkISBN13(isbn string) bool {
	if !isISBNDigit(isbn) {
		return false
	}
	sum := 0
	for i := 0; i < 13; i++ {
		digit := int(isbn[i] - '0')
		if i%2 == 0 {
			sum += digit
		} else {
			sum += 3 * digit
		}
	}
	return sum%10 == 0
}

func isISBNDigit(s string) bool {
	for _, ch := range s {
		if ch < '0' || ch > '9' {
			return false
		}
	}
	return true
}

func (c *Catalog) addBook(isbn, title, author string, year int, publisher string) (int, error) {
	if !c.isValidISBN(isbn) {
		return 0, fmt.Errorf("неверный ISBN")
	}
	for _, b := range c.Books {
		if b.ISBN == isbn {
			return 0, fmt.Errorf("книга с таким ISBN уже существует")
		}
	}
	id := len(c.Books) + 1
	c.Books = append(c.Books, Book{ID: id, ISBN: isbn, Title: title, Author: author, Year: year, Publisher: publisher})
	c.save()
	return id, nil
}

func (c *Catalog) listAll() {
	if len(c.Books) == 0 {
		fmt.Println("\u001B[33mКаталог пуст.\u001B[0m")
		return
	}
	fmt.Printf("\u001B[36m%-4s %-15s %-30s %-20s %-6s %-20s\u001B[0m\n", "ID", "ISBN", "Название", "Автор", "Год", "Издательство")
	fmt.Println(strings.Repeat("-", 100))
	for _, b := range c.Books {
		title := b.Title
		if len(title) > 30 {
			title = title[:30]
		}
		author := b.Author
		if len(author) > 20 {
			author = author[:20]
		}
		pub := b.Publisher
		if len(pub) > 20 {
			pub = pub[:20]
		}
		fmt.Printf("%-4d %-15s %-30s %-20s %-6d %-20s\n", b.ID, b.ISBN, title, author, b.Year, pub)
	}
}

func (c *Catalog) searchByISBN(isbn string) *Book {
	isbn = strings.ReplaceAll(isbn, "-", "")
	isbn = strings.ReplaceAll(isbn, " ", "")
	for _, b := range c.Books {
		if b.ISBN == isbn {
			return &b
		}
	}
	return nil
}

func (c *Catalog) searchByText(text string) []Book {
	text = strings.ToLower(text)
	results := []Book{}
	for _, b := range c.Books {
		if strings.Contains(strings.ToLower(b.Title), text) || strings.Contains(strings.ToLower(b.Author), text) {
			results = append(results, b)
		}
	}
	return results
}

func (c *Catalog) delete(id int) bool {
	for i, b := range c.Books {
		if b.ID == id {
			c.Books = append(c.Books[:i], c.Books[i+1:]...)
			c.save()
			return true
		}
	}
	return false
}

func (c *Catalog) edit(id int, field, value string) error {
	for i, b := range c.Books {
		if b.ID == id {
			switch field {
			case "isbn":
				if !c.isValidISBN(value) {
					return fmt.Errorf("неверный ISBN")
				}
				for _, other := range c.Books {
					if other.ID != id && other.ISBN == value {
						return fmt.Errorf("ISBN уже используется")
					}
				}
				c.Books[i].ISBN = value
			case "title":
				c.Books[i].Title = value
			case "author":
				c.Books[i].Author = value
			case "year":
				y, err := strconv.Atoi(value)
				if err != nil {
					return fmt.Errorf("неверный год")
				}
				c.Books[i].Year = y
			case "publisher":
				c.Books[i].Publisher = value
			default:
				return fmt.Errorf("неизвестное поле")
			}
			c.save()
			return nil
		}
	}
	return fmt.Errorf("книга не найдена")
}

func (c *Catalog) stats() {
	if len(c.Books) == 0 {
		fmt.Println("Нет данных.")
		return
	}
	total := len(c.Books)
	authors := make(map[string]int)
	years := make(map[int]int)
	for _, b := range c.Books {
		authors[b.Author]++
		years[b.Year]++
	}
	fmt.Println("\u001B[36m📊 Статистика:\u001B[0m")
	fmt.Printf("  Всего книг: %d\n", total)
	fmt.Println("  По авторам:")
	for a, cnt := range authors {
		fmt.Printf("    %s: %d\n", a, cnt)
	}
	fmt.Println("  По годам:")
	for y, cnt := range years {
		fmt.Printf("    %d: %d\n", y, cnt)
	}
}

func main() {
	catalog := NewCatalog("books.json")
	reader := bufio.NewReader(os.Stdin)
	for {
		fmt.Println("\n\u001B[36m📚 Book Catalog (ISBN) (Go)\u001B[0m")
		fmt.Println("1. Добавить книгу")
		fmt.Println("2. Показать все книги")
		fmt.Println("3. Поиск по ISBN")
		fmt.Println("4. Поиск по названию/автору")
		fmt.Println("5. Удалить книгу")
		fmt.Println("6. Редактировать книгу")
		fmt.Println("7. Статистика")
		fmt.Println("8. Выход")
		fmt.Print("Выберите действие: ")
		choice, _ := reader.ReadString('\n')
		choice = strings.TrimSpace(choice)
		switch choice {
		case "1":
			fmt.Print("ISBN (10 или 13 цифр): ")
			isbn, _ := reader.ReadString('\n')
			isbn = strings.TrimSpace(isbn)
			fmt.Print("Название: ")
			title, _ := reader.ReadString('\n')
			title = strings.TrimSpace(title)
			fmt.Print("Автор: ")
			author, _ := reader.ReadString('\n')
			author = strings.TrimSpace(author)
			fmt.Print("Год: ")
			yearStr, _ := reader.ReadString('\n')
			year, _ := strconv.Atoi(strings.TrimSpace(yearStr))
			fmt.Print("Издательство: ")
			pub, _ := reader.ReadString('\n')
			pub = strings.TrimSpace(pub)
			id, err := catalog.addBook(isbn, title, author, year, pub)
			if err != nil {
				fmt.Printf("\u001B[31m❌ Ошибка: %v\u001B[0m\n", err)
			} else {
				fmt.Printf("\u001B[32m✅ Книга добавлена (ID: %d)\u001B[0m\n", id)
			}
		case "2":
			catalog.listAll()
		case "3":
			fmt.Print("Введите ISBN: ")
			isbn, _ := reader.ReadString('\n')
			isbn = strings.TrimSpace(isbn)
			book := catalog.searchByISBN(isbn)
			if book != nil {
				fmt.Printf("ID: %d\nISBN: %s\nНазвание: %s\nАвтор: %s\nГод: %d\nИздательство: %s\n",
					book.ID, book.ISBN, book.Title, book.Author, book.Year, book.Publisher)
			} else {
				fmt.Println("\u001B[33mКнига не найдена.\u001B[0m")
			}
		case "4":
			fmt.Print("Введите название или автора: ")
			text, _ := reader.ReadString('\n')
			text = strings.TrimSpace(text)
			results := catalog.searchByText(text)
			if len(results) > 0 {
				for _, b := range results {
					fmt.Printf("%d: %s | %s | %d\n", b.ID, b.Title, b.Author, b.Year)
				}
			} else {
				fmt.Println("\u001B[33mНичего не найдено.\u001B[0m")
			}
		case "5":
			catalog.listAll()
			fmt.Print("Введите ID для удаления: ")
			idStr, _ := reader.ReadString('\n')
			id, _ := strconv.Atoi(strings.TrimSpace(idStr))
			if catalog.delete(id) {
				fmt.Println("\u001B[32m✅ Книга удалена.\u001B[0m")
			} else {
				fmt.Println("\u001B[31m❌ Книга не найдена.\u001B[0m")
			}
		case "6":
			catalog.listAll()
			fmt.Print("Введите ID для редактирования: ")
			idStr, _ := reader.ReadString('\n')
			id, _ := strconv.Atoi(strings.TrimSpace(idStr))
			fmt.Print("Какое поле редактировать (isbn, title, author, year, publisher): ")
			field, _ := reader.ReadString('\n')
			field = strings.TrimSpace(strings.ToLower(field))
			fmt.Print("Новое значение: ")
			value, _ := reader.ReadString('\n')
			value = strings.TrimSpace(value)
			err := catalog.edit(id, field, value)
			if err != nil {
				fmt.Printf("\u001B[31m❌ Ошибка: %v\u001B[0m\n", err)
			} else {
				fmt.Println("\u001B[32m✅ Книга обновлена.\u001B[0m")
			}
		case "7":
			catalog.stats()
		case "8":
			fmt.Println("До свидания!")
			return
		default:
			fmt.Println("\u001B[31mНеверный выбор.\u001B[0m")
		}
	}
}
