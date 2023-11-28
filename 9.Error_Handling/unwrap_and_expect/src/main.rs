use std::fs::File;
use std::io::{self, Read};

fn main() {
    // Result<T, E> 타입은 다양한 특정 작업을 수행하기 위해 정의된 수많은 도우미 메서드를 가지고
    // 있다. Result 값이 Ok variant라면, unwrap은 Ok 내의 값을 반환할 것이다.
    // Result가 Err variant라면, unwrap은 panice! macro를 호출해 줄 것이다.

    let greeting_file = File::open("hello.txt").unwrap();

    // 이와 비슷한 expect는 panic! error 메시지도 선택할 수 있도록 해준다.

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    // unwrap은 panic!의 기본 메시지가 출력되지만, expect는 매개변수로 전달한 에러 메시지를
    // 출력한다. 대부분의 러스타시안은 expect를 선택한다,


    // Propagating Errors:
    // 함수의 구현체에서 실패할 수도 있는 무언가를 호출할 때, 이 함수에서 에러를 처리하는 대신
    // 이 함수를 호출하는 코드 쪽으로 에러를 반환하여 그쪽에서 수행할 작업을 결정하도록
    // 할 수 있다. 이를 에러 propagating이라 하며, 호출하는 코드 쪽에 더 많은 제어권을 주는 것이다.
    // 호출하는 코드 쪽에는 에러를 어떻게 처리해야 하는지 결정하는 정보와 로직이
    // 우리의 코드 컨텍스트 내에서 활용할 수 있는 것보다 더 많이 있을 수도 있기 때문이다.


    // String은 concrete type, 컴파일 시점에 구체적으로 명시된 타입이다.
    // generic type은 abstract type, 컴파일 시에 정확한 타입이 결정되지 않은 경우를 나타낸다.

    fn read_username_from_file() -> Result<String, io::Error> {
        // Result<T, E>의 E type으로 io::Error을 선택한 이유는 함수 내부에서 호출하는 실패할 수
        // 있는 연산 File::open 함수와 read_to_string 메서드 두 가지 모두 io::Error 타입의 에러
        // 값을 반환하기 때문이다.

        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),  
            // Err의 경우 panic!을 호출하는 대신 return 키워드를 사용해
            // 함수 전체를 일찍 끝내고 호출한 코드 쪽에 File::open으로부터 얻은(지금의 경우 패턴
            // 변수 e에 있는) 에러 값을 이 함수의 에러 값처럼 넘긴다.
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {  
            // username_file은 파일 핸들로, 해당 파일의 내용물을 username에 할당하기 위해 
            // read_to_string 메서드를 이용한다.
            // File::open이 성공하더라도 .read_to_string이 실패할 수 있으므로 Result를 반환한다.
            Ok(_) => Ok(username),
            Err(e) => Err(e),  // Error 값 반환한다.
        }

        // 이 코드를 호출하는 코드는 사용자이름이 있는 Ok 값 혹은 io::Error를 담은 Err값을 처리하게
        // 될 것이다. 이 값을 가지고 어떤 일을 할지에 대한 결정은 호출하는 코드 쪽에 달려 있다.
        // 호출하는 코드에서 panic!을 호출해 프로그램을 종료시킬 수도, 기본 사용자 이름을 사용할
        // 수도 있으며, 파일이 아닌 다른 어딘가에서 사용자 이름을 찾을 수도 있다.
        // 호출하는 코드가 정확이 어떤 것을 시도하려 하는지에 대한 충분한 정보가 없기 때문에,
        // 모든 성공 혹은 에러 정보를 위로 전파하여 호출하는 코드가 적절하게 처리하도록 한다.
        // Rust에는 Error을 propagate하는 패턴이 너무 흔하여 이를 더 쉽게 해주는 ? 연산자를 제공한다.
    } 
}
