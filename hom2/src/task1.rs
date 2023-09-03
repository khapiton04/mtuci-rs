/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
номерстроки: строка...

 */

const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

fn main() {
    let result = find_term(SEARCH_TERM, QUOTE);
    println!("{}", result);
}

fn find_term(search_term: &str, quote: &str) -> String {
    let mut line_number = 0;
    for line in quote.lines() {
        line_number += 1;
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in words {
            if word == search_term {
                return format!("{}: {}", line_number, line.trim());
            }
        }
    }
    String::from("Word not found.")
}

//----> TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);
        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer);
    }
}
