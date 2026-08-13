# catalog.rb — Ruby версия

require 'json'
require 'date'

DATA_FILE = 'books.json'

class Book
  attr_accessor :id, :isbn, :title, :author, :year, :publisher

  def initialize(id, isbn, title, author, year, publisher)
    @id = id
    @isbn = isbn
    @title = title
    @author = author
    @year = year
    @publisher = publisher
  end

  def to_h
    { id: @id, isbn: @isbn, title: @title, author: @author, year: @year, publisher: @publisher }
  end

  def self.from_h(h)
    new(h[:id], h[:isbn], h[:title], h[:author], h[:year], h[:publisher])
  end
end

class BookCatalog
  attr_reader :books

  def initialize
    @books = []
    load
  end

  def load
    if File.exist?(DATA_FILE)
      begin
        data = JSON.parse(File.read(DATA_FILE), symbolize_names: true)
        @books = data.map { |b| Book.from_h(b) }
      rescue
        @books = []
      end
    end
  end

  def save
    File.write(DATA_FILE, JSON.pretty_generate(@books.map(&:to_h)))
  end

  def is_valid_isbn?(isbn)
    isbn = isbn.gsub('-', '').gsub(' ', '')
    if isbn.length == 10
      check_isbn10(isbn)
    elsif isbn.length == 13
      check_isbn13(isbn)
    else
      false
    end
  end

  def check_isbn10(isbn)
    return false unless isbn =~ /^\d{9}[\dX]$/
    sum = (0...9).sum { |i| (i + 1) * isbn[i].to_i }
    check = isbn[9]
    sum += check == 'X' ? 100 : 10 * check.to_i
    sum % 11 == 0
  end

  def check_isbn13(isbn)
    return false unless isbn =~ /^\d{13}$/
    sum = isbn.chars.each_with_index.sum do |c, i|
      digit = c.to_i
      i.even? ? digit : 3 * digit
    end
    sum % 10 == 0
  end

  def add_book(isbn, title, author, year, publisher)
    unless is_valid_isbn?(isbn)
      raise "Неверный ISBN"
    end
    if @books.any? { |b| b.isbn == isbn }
      raise "Книга с таким ISBN уже существует"
    end
    id = @books.size + 1
    @books << Book.new(id, isbn, title, author, year, publisher)
    save
    id
  end

  def list_all
    if @books.empty?
      puts "\e[33mКаталог пуст.\e[0m"
      return
    end
    printf "\e[36m%-4s %-15s %-30s %-20s %-6s %-20s\e[0m\n", "ID", "ISBN", "Название", "Автор", "Год", "Издательство"
    puts "-" * 100
    @books.each do |b|
      title = b.title.length > 30 ? b.title[0...30] : b.title
      author = b.author.length > 20 ? b.author[0...20] : b.author
      pub = b.publisher.length > 20 ? b.publisher[0...20] : b.publisher
      printf "%-4d %-15s %-30s %-20s %-6d %-20s\n", b.id, b.isbn, title, author, b.year, pub
    end
  end

  def search_by_isbn(isbn)
    isbn = isbn.gsub('-', '').gsub(' ', '')
    @books.find { |b| b.isbn == isbn }
  end

  def search_by_text(text)
    text = text.downcase
    @books.select { |b| b.title.downcase.include?(text) || b.author.downcase.include?(text) }
  end

  def delete(id)
    found = @books.find { |b| b.id == id }
    if found
      @books.delete(found)
      save
      true
    else
      false
    end
  end

  def edit(id, field, value)
    book = @books.find { |b| b.id == id }
    raise "Книга не найдена" unless book
    case field
    when 'isbn'
      unless is_valid_isbn?(value)
        raise "Неверный ISBN"
      end
      if @books.any? { |b| b.id != id && b.isbn == value }
        raise "ISBN уже используется"
      end
      book.isbn = value
    when 'title'
      book.title = value
    when 'author'
      book.author = value
    when 'year'
      book.year = value.to_i
    when 'publisher'
      book.publisher = value
    else
      raise "Неизвестное поле"
    end
    save
    true
  end

  def stats
    if @books.empty?
      puts "Нет данных."
      return
    end
    authors = Hash.new(0)
    years = Hash.new(0)
    @books.each do |b|
      authors[b.author] += 1
      years[b.year] += 1
    end
    puts "\e[36m📊 Статистика:\e[0m"
    puts "  Всего книг: #{@books.size}"
    puts "  По авторам:"
    authors.sort_by { |_, c| -c }.each do |a, c|
      puts "    #{a}: #{c}"
    end
    puts "  По годам:"
    years.sort.each do |y, c|
      puts "    #{y}: #{c}"
    end
  end
end

def main
  catalog = BookCatalog.new
  loop do
    puts "\n\e[36m📚 Book Catalog (ISBN) (Ruby)\e[0m"
    puts "1. Добавить книгу"
    puts "2. Показать все книги"
    puts "3. Поиск по ISBN"
    puts "4. Поиск по названию/автору"
    puts "5. Удалить книгу"
    puts "6. Редактировать книгу"
    puts "7. Статистика"
    puts "8. Выход"
    print "Выберите действие: "
    choice = gets.chomp
    case choice
    when "1"
      print "ISBN (10 или 13 цифр): "
      isbn = gets.chomp
      print "Название: "
      title = gets.chomp
      print "Автор: "
      author = gets.chomp
      print "Год: "
      year = gets.chomp.to_i
      print "Издательство: "
      publisher = gets.chomp
      begin
        id = catalog.add_book(isbn, title, author, year, publisher)
        puts "\e[32m✅ Книга добавлена (ID: #{id})\e[0m"
      rescue => e
        puts "\e[31m❌ Ошибка: #{e.message}\e[0m"
      end
    when "2"
      catalog.list_all
    when "3"
      print "Введите ISBN: "
      isbn = gets.chomp
      book = catalog.search_by_isbn(isbn)
      if book
        puts "ID: #{book.id}\nISBN: #{book.isbn}\nНазвание: #{book.title}\nАвтор: #{book.author}\nГод: #{book.year}\nИздательство: #{book.publisher}"
      else
        puts "\e[33mКнига не найдена.\e[0m"
      end
    when "4"
      print "Введите название или автора: "
      text = gets.chomp
      results = catalog.search_by_text(text)
      if results.any?
        results.each { |b| puts "#{b.id}: #{b.title} | #{b.author} | #{b.year}" }
      else
        puts "\e[33mНичего не найдено.\e[0m"
      end
    when "5"
      catalog.list_all
      print "Введите ID для удаления: "
      id = gets.chomp.to_i
      if catalog.delete(id)
        puts "\e[32m✅ Книга удалена.\e[0m"
      else
        puts "\e[31m❌ Книга не найдена.\e[0m"
      end
    when "6"
      catalog.list_all
      print "Введите ID для редактирования: "
      id = gets.chomp.to_i
      print "Какое поле редактировать (isbn, title, author, year, publisher): "
      field = gets.chomp.downcase
      print "Новое значение: "
      value = gets.chomp
      begin
        catalog.edit(id, field, value)
        puts "\e[32m✅ Книга обновлена.\e[0m"
      rescue => e
        puts "\e[31m❌ Ошибка: #{e.message}\e[0m"
      end
    when "7"
      catalog.stats
    when "8"
      puts "До свидания!"
      break
    else
      puts "\e[31mНеверный выбор.\e[0m"
    end
  end
end

main if __FILE__ == $0
