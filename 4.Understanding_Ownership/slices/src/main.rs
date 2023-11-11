/*
스트링을 입력 받아 그 스트링에서 찾은 첫번째 단어를 반환하는 함수를 작성하라.
만일 함수가 공박문자를 찾지 못한다면, 이는  전체 스트링이 한 단어라는 의미이고,
이때는 전체 스트링이 반환되어야 한다.
*/

fn main() {
    let mut s = String::from("hello world!");
    let word = first_word(&s); // word는 5를 갖게 될 것입니다.
    s.clear(); // 이 코드는 String을 비워서 ""로 만들게 됩니다.

    println!("word is: {word}");

    // word는 여기서 여전히 5를 갖고 있지만, 5라는 값을 의미있게 쓸 수 있는 스트링은 이제 없습니다.
    // word는 이제 완전 유효하지 않습니다!
    
    // word의 인덱스가 s의 데이터와 싱크가 안맞을 것을 걱정하는 건 지겹고 쉽게 발생할 수 있는
    // 오류이다. 동기화를 유지할 수 없을 때 인덱스가 늘어날 수록 문제가 커진다.
    
    // 이렇 때 사용하는 방법이 String Slice이다.
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{  // 단일 바이트 문자 리터럴 b''
            return i;  
            // 바이트 리터럴 문법을 이용하여 공백 문자를 나타내는 바이트를 찾는다.
            // 공백 문자를 찾았다면, 이 위치를 반환한다.
        }
    }
    s.len()
    // 그렇지 않으면 s.len()을 통해 스트링의 길이값을 반환한다.
}

