use std::fs::<self, File>;
use std::io::<self, Read>;

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // lines는 받은 &str 문자열의 라인에 대한 반복자 반환
    // next()는 첫 번째 값을 얻어옴.
    // chars()는 text의 첫 번째 줄의 문자열 슬라이스를
    // 담고 있는 Some의 값을 받아 iterator로 변환
    // last를 호출해 이 iterator의 last item 얻어옴.
    // "\nhi"처럼 빈 줄로 시작하지만
    // 다른 줄에는 문자가 담겨있는 경우처럼,
    // 첫 번째 라인이 빈 문자열일 수 있으므로 
    // iterator returns Option
    // it'll return Some variant 
    // if there's a last char in first line.
    
    // Result를 반환하는 함수에서는 Option에 대해 
    // ? 연산자를 사용할 수 없다.
    // 즉, Result와 Option 섞어서 사용 불가능.
    // 연산자는 자동으로 Result를 Option으로 변환하거나
    // 혹은 그 반대를 할 수 없다;
    // 그런 경우 Result의 ok() 메서드나 Option의 ok_or
    // 메서드 같은 것을 통해 명시적으로 반환할 수 있다.
}
fn main() -> Result<(), Box<dyn Error>>> {
    // Box<dyn Error>은 trait object이다.
    // 이는 어떠한 에러를 의미한다.
    // main 함수가 Result<(), E>를 반환하게 되면,
    // 실행 파일은 main이 Ok(())를 반환할 경우 0으로,
    // main이 Err을 반환할 경우 0이 아닌 값으로 종료된다.

    let greeting_file = File::open("hi.txt")?;
    
    Ok(())

    // main 함수가 std::process::Termination trait를 
    // 구현한 타입을 반환할 수도 있는데,
    // 이는 ExitCode를 반환하는 report라는 함수를
    // 가지고 있다.
    // Termination를 다루는 표준 라이브러리 문서를 찾아보기
}
