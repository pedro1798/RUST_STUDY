use std::io;

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    // bool type return하는 matches! macro 
}

fn pig_latin_word(word: &str) -> String {
    let mut chars = word.chars();

    match chars.next() {  
        // next()는 Iterator에서 다음 요소를 가져온다.
        // Iterator에서 요소를 가져오면 해당 요소는 Iterator에서 제거된다.
        // next()는 Option<T>를 반환한다. Some<T>는 다음 요소가 있음을, None은 더 이상 요소가
        // 없음을 나타낸다.
        Some(first_char) if is_vowel(first_char) => {
            // 모음으로 시작하는 경우
            format!("{}-hay", word)
        }
        Some(first_char) => {
            // 자음으로 시작하는 경우
            let rest_of_word: String = chars.collect();
            format!("{}-{}ay", rest_of_word, first_char)
        } 
        None => String::new(), // 빈 문자열 처리
    }
}

fn pig_latin_sentence(sentence: &str) -> String {
    // 공백 기준으로 단어 분리하고 Pig Latin으로 변환
    sentence
        .split_whitespace()  // std::str::SplitWhitespace Iterator을 반환한다.
        .map(pig_latin_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let mut input_sentence = String::new();
    io::stdin().read_line(&mut input_sentence)
        .expect("Incorrect input");

    let pig_latin_result = pig_latin_sentence(&input_sentence);

    println!("Input sentence: {}", input_sentence);
    println!("Pig Latin Result: {}", pig_latin_result);
}
