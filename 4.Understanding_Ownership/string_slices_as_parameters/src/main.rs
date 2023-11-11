fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);
    // first_word가 String의 슬라이스로 동작한다.
    
    let my_string_literal = "hello world!";

    let word = first_word(&my_string_literal[..]);
    // first_word가 스트링 리터럴의 슬라이스로 동작한다.
    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작한다.
    let word = first_word(my_string_literal);

    // 그 밖의 술라이스들...
    
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    // 배열의 일부를 참조하는 슬라이스 타입도 존재한다.
    // 이 슬라이스는 &[i32] 타입을 갖는다. 이는 스트링 슬라이스가 동작하는방법과 똑같이, 슬라이스의
    // 첫번째 요소에 대한 참조자와 슬라이스의 길이를 저장하는 방식으로 동작한다.
    // 다른 모든 종류의 컬렉션들에 대해 이런 종류의 슬라이스를 이용할 수 있다. 이는 8장에서 다룬다.
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    
    for (i, &ite)m in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

// 더 경험이 많은 러스트인이라면 parameter를 (s: &String) 대신 (s: &str)와 같이 작성하는데,
// 그 이유는 &String과 &str 둘 모두에 대한 같은 함수를 사용할 수 있도록 해주기 때문이다.
// 만일 우리가 스트링 슬라이스를 갖고 있다면, 이를 바로 넘길 수 있다.
// String을 갖고 있다면, 이 String의 전체 슬라이스를 넘길 수 있다.
// &str과 &String[..]을 넘길 수 있다는 이야기.
// 함수가 String의 참조자 대신 스트링 슬라이스를 갖도록 정의하는 것은 우리의 API를 어떠한 기능적인
// 손실 없이도 더 일반적이고 유용하게 해준다.
