use std::io;
use std::fs::File;

fn main() {
    // Sometimes, when a function fails, it's for a reason that you can easily interpret and 
    // respond to.
    let mut a = String::new();
    let b  = io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    println!("a is: {a}, byte_num_of_a is: {b}");
    // 하나의 긴 라인은 가독성이 떨어지므로 라인을 나누는 것이 좋다.
    // .method_name() 문법으로 어떤 메서드를 호출할 때는 특히 더.

    // read_line은 우리가 인수로 넘긴 문자열에 사용자가 입력한 것을 저장할 뿐만 아니라,
    // 하나의 Result 값을 돌려준다.
    // Result는 enum이라고도 일컫는 열거형인데, 여러 개의 가능한 상태 중 하나의 값이 될 수 있는
    // 타입이다. 이러한 가능한 상태 값을 variant라고 부른다.
    // Result의 variant는 Ok와 Err이다. Result 타입의 값에도 메서드가 있다.
    // 이는 expect로, Result 인스턴스가 Err일 경우 expect 메서드는 프로그램의 작동을 멈추고
    // expect에 인수로 넘겼던 메시지를 출력하도록 한다.
    // 만약 Result가 Ok 값이라면, expect는 Ok가 가지고 있는 결괏값을 돌려주어 
    // 사용할 수 있도록 한다. 위의 경우 결괏값은 사용자가 표준 입력으로 입력했던 바이트의 개수이다.
    

    enum Result<T, E> {  // Result enumeration의 definition
        Ok(T),  // T는 성공한 경우에 Ok variant 안에 반환될 값의 타입.
        Err(E),  // E는 실패한 경우에 Err variant 안에 반환될 에러의 타입.
    // Result가 generic type parameter를 갖기 때문에, 반환하고자 하는 성공적인 값과 에러 값이
    // 달라질 수 있는 다양한 상황에서 Result 타입 및 이에 정의된 함수들을 사용할 수 있다.
    }

    let greeting_file_result = File::open("hello.txt");  // 파일을 열어보는 코드이다.
    let greeting_file_result = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // The return type of File::open is a Result<T, E>. 
    // The generic parameter T has been filled in the implementation of File::open with the type of
    // the success value, std::fs::File, which is a file handle.
    // The type of E used in the error value is std::io::Error. This return type means the call to
    // File::open might succeed and return a file handle that we can read from or write to.

    // Option enumeration과 같이, Result와 variant는 Rust의 prelude에서 가져와진다.
    // 따라서 match 갈래의 Ok와 Error 앞에 Result::라고 지정하지 않아도 된다.
}
