use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");  // 반환하는 Err variant값의 type은 io::Error
    // 이는 표준 라이브러리에서 제공하는 구조체이다.
    // 이 구조체가 제공하는 .kind() 메서드를 호출하여 io::ErrorKind 값을 얻을 수 있다.
    // 표준 라이브러리가 제공하는 io::ErrorKind는 io연산으로부터 발생할 수 있는 다양한 종류의
    // 에러를 나타내는 variant가 있는 열거형이다.
    // 여기서 사용하고자 하는 variant는 ErrorKind::NotFound이며, 열고자 하는 파일이 아직 존재하지
    // 않음을 나타낸다. 따라서 error.kind()에 대한 내부 매칭이 하나 더 생긴다.

    
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {  // 반환한 값이 ErrorKind enum의 NotFound variant가 맞는지 확인
            ErrorKind::NotFound => match File::create("hello.txt") {  // 맞다면 파일 생성.
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),  // 파일 생성 실패 시 에러 메시지 출력
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        },
    };
    

    // match는 primitive함. 더 나은 방식:

    let greeting_file = File::open("hello.txt")
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt")
                    .unwrap_or_else(|error| {
                        panic!("Problem creating the file: {:?}", error);
                    })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        }); 
}
