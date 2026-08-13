<?php
// catalog.php — PHP версия

$dataFile = 'books.json';

function loadBooks() {
    global $dataFile;
    if (file_exists($dataFile)) {
        $json = file_get_contents($dataFile);
        return json_decode($json, true) ?: [];
    }
    return [];
}

function saveBooks($books) {
    global $dataFile;
    file_put_contents($dataFile, json_encode($books, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE));
}

$books = loadBooks();

function color($text, $code) {
    return "\033[{$code}m{$text}\033[0m";
}

function isValidISBN($isbn) {
    $isbn = str_replace(['-', ' '], '', $isbn);
    if (strlen($isbn) == 10) return checkISBN10($isbn);
    if (strlen($isbn) == 13) return checkISBN13($isbn);
    return false;
}

function checkISBN10($isbn) {
    if (!preg_match('/^\d{9}[\dX]$/', $isbn)) return false;
    $sum = 0;
    for ($i = 0; $i < 9; $i++) {
        $sum += ($i + 1) * intval($isbn[$i]);
    }
    $check = $isbn[9];
    if ($check == 'X') $sum += 100;
    else $sum += 10 * intval($check);
    return $sum % 11 == 0;
}

function checkISBN13($isbn) {
    if (!preg_match('/^\d{13}$/', $isbn)) return false;
    $sum = 0;
    for ($i = 0; $i < 13; $i++) {
        $digit = intval($isbn[$i]);
        $sum += ($i % 2 == 0) ? $digit : 3 * $digit;
    }
    return $sum % 10 == 0;
}

function listAll($books) {
    if (empty($books)) {
        echo color("Каталог пуст.\n", '33');
        return;
    }
    printf(color("%-4s %-15s %-30s %-20s %-6s %-20s\n", '36'), "ID", "ISBN", "Название", "Автор", "Год", "Издательство");
    echo str_repeat("-", 100) . "\n";
    foreach ($books as $b) {
        $title = strlen($b['title']) > 30 ? substr($b['title'], 0, 30) : $b['title'];
        $author = strlen($b['author']) > 20 ? substr($b['author'], 0, 20) : $b['author'];
        $pub = strlen($b['publisher']) > 20 ? substr($b['publisher'], 0, 20) : $b['publisher'];
        printf("%-4d %-15s %-30s %-20s %-6d %-20s\n", $b['id'], $b['isbn'], $title, $author, $b['year'], $pub);
    }
}

function searchByISBN($books, $isbn) {
    $isbn = str_replace(['-', ' '], '', $isbn);
    foreach ($books as $b) {
        if ($b['isbn'] == $isbn) return $b;
    }
    return null;
}

function searchByText($books, $text) {
    $text = strtolower($text);
    $results = [];
    foreach ($books as $b) {
        if (stripos($b['title'], $text) !== false || stripos($b['author'], $text) !== false) {
            $results[] = $b;
        }
    }
    return $results;
}

function deleteBook(&$books, $id) {
    foreach ($books as $i => $b) {
        if ($b['id'] == $id) {
            array_splice($books, $i, 1);
            saveBooks($books);
            return true;
        }
    }
    return false;
}

function editBook(&$books, $id, $field, $value) {
    foreach ($books as &$b) {
        if ($b['id'] == $id) {
            switch ($field) {
                case 'isbn':
                    if (!isValidISBN($value)) throw new Exception("Неверный ISBN");
                    foreach ($books as $other) {
                        if ($other['id'] != $id && $other['isbn'] == $value) throw new Exception("ISBN уже используется");
                    }
                    $b['isbn'] = $value;
                    break;
                case 'title': $b['title'] = $value; break;
                case 'author': $b['author'] = $value; break;
                case 'year': $b['year'] = (int)$value; break;
                case 'publisher': $b['publisher'] = $value; break;
                default: throw new Exception("Неизвестное поле");
            }
            saveBooks($books);
            return true;
        }
    }
    throw new Exception("Книга не найдена");
}

function stats($books) {
    if (empty($books)) {
        echo "Нет данных.\n";
        return;
    }
    $authors = [];
    $years = [];
    foreach ($books as $b) {
        $authors[$b['author']] = ($authors[$b['author']] ?? 0) + 1;
        $years[$b['year']] = ($years[$b['year']] ?? 0) + 1;
    }
    echo color("📊 Статистика:\n", '36');
    echo "  Всего книг: " . count($books) . "\n";
    echo "  По авторам:\n";
    arsort($authors);
    foreach ($authors as $a => $c) {
        echo "    $a: $c\n";
    }
    echo "  По годам:\n";
    ksort($years);
    foreach ($years as $y => $c) {
        echo "    $y: $c\n";
    }
}

function main() {
    global $books;
    while (true) {
        echo "\n" . color("📚 Book Catalog (ISBN) (PHP)\n", '36');
        echo "1. Добавить книгу\n";
        echo "2. Показать все книги\n";
        echo "3. Поиск по ISBN\n";
        echo "4. Поиск по названию/автору\n";
        echo "5. Удалить книгу\n";
        echo "6. Редактировать книгу\n";
        echo "7. Статистика\n";
        echo "8. Выход\n";
        echo "Выберите действие: ";
        $choice = trim(fgets(STDIN));

        switch ($choice) {
            case '1':
                echo "ISBN (10 или 13 цифр): ";
                $isbn = trim(fgets(STDIN));
                echo "Название: ";
                $title = trim(fgets(STDIN));
                echo "Автор: ";
                $author = trim(fgets(STDIN));
                echo "Год: ";
                $year = (int) trim(fgets(STDIN));
                echo "Издательство: ";
                $publisher = trim(fgets(STDIN));
                if (!isValidISBN($isbn)) {
                    echo color("❌ Неверный ISBN.\n", '31');
                    break;
                }
                foreach ($books as $b) {
                    if ($b['isbn'] == $isbn) {
                        echo color("❌ Книга с таким ISBN уже существует.\n", '31');
                        break 2;
                    }
                }
                $id = count($books) + 1;
                $books[] = ['id' => $id, 'isbn' => $isbn, 'title' => $title, 'author' => $author, 'year' => $year, 'publisher' => $publisher];
                saveBooks($books);
                echo color("✅ Книга добавлена (ID: $id)\n", '32');
                break;
            case '2':
                listAll($books);
                break;
            case '3':
                echo "Введите ISBN: ";
                $isbn = trim(fgets(STDIN));
                $book = searchByISBN($books, $isbn);
                if ($book) {
                    echo "ID: {$book['id']}\nISBN: {$book['isbn']}\nНазвание: {$book['title']}\nАвтор: {$book['author']}\nГод: {$book['year']}\nИздательство: {$book['publisher']}\n";
                } else {
                    echo color("Книга не найдена.\n", '33');
                }
                break;
            case '4':
                echo "Введите название или автора: ";
                $text = trim(fgets(STDIN));
                $results = searchByText($books, $text);
                if (empty($results)) {
                    echo color("Ничего не найдено.\n", '33');
                } else {
                    foreach ($results as $b) {
                        echo "{$b['id']}: {$b['title']} | {$b['author']} | {$b['year']}\n";
                    }
                }
                break;
            case '5':
                listAll($books);
                echo "Введите ID для удаления: ";
                $id = (int) trim(fgets(STDIN));
                if (deleteBook($books, $id)) {
                    echo color("✅ Книга удалена.\n", '32');
                } else {
                    echo color("❌ Книга не найдена.\n", '31');
                }
                break;
            case '6':
                listAll($books);
                echo "Введите ID для редактирования: ";
                $id = (int) trim(fgets(STDIN));
                echo "Какое поле редактировать (isbn, title, author, year, publisher): ";
                $field = trim(fgets(STDIN));
                echo "Новое значение: ";
                $value = trim(fgets(STDIN));
                try {
                    editBook($books, $id, $field, $value);
                    echo color("✅ Книга обновлена.\n", '32');
                } catch (Exception $e) {
                    echo color("❌ Ошибка: " . $e->getMessage() . "\n", '31');
                }
                break;
            case '7':
                stats($books);
                break;
            case '8':
                echo "До свидания!\n";
                exit(0);
            default:
                echo color("Неверный выбор.\n", '31');
        }
    }
}

main();
?>
