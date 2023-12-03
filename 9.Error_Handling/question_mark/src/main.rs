use std::fs::{self, File};
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, OurError>{
    let mut username_file = File::open("hello.txt")?;
    // Result 값이 Ok라면, Ok 안의 값이 얻어지고
    // 프로그램은 계속된다.
    // 값이 Err라면, return keyword로 에러 값을
    // 호출하는 코드에게 전파하는 것처럼 Err이 반환된다.

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    // 이렇게 위 코드를 두 줄로  단축할 수 있다.
    let mut username = String::new();
    File::open()?.read_to_string(&mut username)?;

    Ok(username)

    // 반환까지 fs::read_to_string을 사용해 단축 가능
    fs::read_to_string("hello.txt")
    /*
     * 표준 라이브러리에서는 파일을 열고, 새 String을
     * 생성하고, 파일 내용을 읽고, 내용을 String에
     * 집어넣고 반환하는 fs::read_to_string라는
     * 편리한 함수를 제공한다.
     */
    
    /*
    match 표현식과 ? 연산자의 차이점:
    ? 연산자를 사용할 때의 에러 값들은 
    from 함수를 거친다. from 함수는 
    표준 라이브러리 내의 From trait에 정의되어 있으며, 
    어떤 값의 타입을 다른 타입으로 변환하는 데에 사용한다.
    ? 연산자가 from 함수를 호출하면, 
    ?연산자가 얻게 되는 에러를 
    ? 연산자가 사용된  현재 함수의
    반환 타입에 정의된 에러 타입으로 변환한다.
    이는 어떤 함수가 다양한 종류의 에러로 인해 
    실패할 수 있지만, 모든 에러를 하나의 에러
    타입으로 반환할 때 유용하다.
    */
    

    // ? 연산자는 ?이 사용된 값과 호환 가능한 반환 타입
    // 을 가진 함수에서만 사용될 수 있다. 이는 ?연산자가
    // match 표현식과 동일한 방식으로 함수를 일찍
    // 끝내면서 값을 반환하는 동작을 수행하도록
    // 정의되어 있기 때문이다.
}

// Our custom error type
#[derive(Debug)]
struct OurError {
    message: String,
}

// Implementing the From trait for from io::Error to OurError
impl From<io::Error> for OurError {
    fn from(error: io::Error) -> Self {
        // Extract information from io::Error or 
        // create a custom message
        let message = format!("Custom error: {}", error);

        // Return a new instance of OutError
        OurError { message }
    }
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("{}", username),
        Err(error) => println!("Error: {:?}", error),
    }

    // let greeting_file = File::open("hello.txt")?;
    // main 함수는 ()를 반환하기 때문에 ? 사용 불가함.
    // ? 연산자는 Result, Option 혹은 FromResidual을
    // 구현한 타입을 반환하는 함수에서만 사용될 수 있다.
}
