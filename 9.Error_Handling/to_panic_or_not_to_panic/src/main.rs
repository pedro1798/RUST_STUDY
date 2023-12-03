fn main() {
    // 한번 panic!을 일으키면 복구할 방법이 없다.
    // Result 값을 반환하는 선택은 호출하는 쪽에 옵션을 제공하는 것이다.
    // 호출한 함수에서 받은 Err을 가지고 복구를 시도할 것인지, panic!을 일으킬 것인지 정할 수 있음.
    // 기본적으로 Result를 반환하는 것이 좋은 선택임.

    // ##############  panic!을 추가해야할 겅우들 가이드라인 ####################

    /* 예제, 프로토타입 코드, 테스트
     *
     * 1. 예제 작성: 
     * When you're writing an example to illustrate some concept, also including robust
     * error-handling code can make the example less clear.
     * In examples, it's understood that a call to a method like unwrap that could panic is meant
     * as a placeholder for the way you'd want your apllication to handle errors,
     * which can differ based on what the rest of your code is doing.
     *
     * 2. prototype code :
     * Similarly, the unwrap and expect methods are very handy when prototyping, before you're
     * ready to decide how to handle errors. They leave clear markers in your code for when you're
     * ready to make your program more robust.
     *
     * 3. tests : 
     * If a method call fails in a test, you'd want the whole tet to fail, even if that method
     * isn't the functionality under test. Because panic! is how a test is marked as a failure,
     * calling unwrap or expect is exactly what should happen.
     *
     * */

    // 우리가 컴파일러보다 더 많은 정보를 가지고 있을 때 :
    /*
     * Result가 Ok값을 가지고 있을 거라 확신할만한 논리적 근거가 있지만, 컴파일러가 그 논리를
     * 이해할 수 없는 경우에 unwrap 혹은 expect를 호출하는 것이 적절할 수 있다.
     * 어떠한 연산이든 간에 특정한 상황에서는 논리적으로 불가능할지라도 기본적으로는 실패할
     * 가능성을 가지고 있는 코드를 호출하는 것이므로, 처리가 필요한 Result 값이 나오게 된다.
     * 손수 코드를 조사하여 Err 배리언트가 나올리 없음을 확신할 수 있다면 unwrap을 호출해도 아무런
     * 문제가 없으며, expect의 문구에 Err 배리언트가 있으면 안 될 이유를 적어주는 것이 더 좋을
     * 것이다. 
     */

    use std::net::IpAddr;
    
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid.");
    // 하드코딩된 문자열을 파싱하여 IpAddr 인스턴스를 만드는 코드이다.
    /*
     * 127.0.0.1이 유효한 IP 주소라는 사실을 알 수 있으므로, 여기선 .expect의 사용이 허용된다.
     * However, having a hardcoded, valid string doesn't change the return type of the parse
     * method: we still get a Result value, and the compiler will still make use handle the Result
     * as if the Err variant is a possibility because the compiler isn't smart enough to see that
     * this string is always a valid IP address.
     * If the IP address string came from a user rather than being hardcoded into the program and
     * therefore did have a possibility of failure, we'd definitely want to handle the Result in a
     * more robust wat instead.
     * Mentioning the assumption that this IP address is hardcoded will prompt us to change expect
     * to better error handling code if in the future, we need to get the IP address from some
     * other source instead.
     */

    // Guideline for Error Handling
    /*
     * 코드가 결국 나쁜 상태(가정, 보장, 계약, 불변성이 깨질 때, 유효하지 않은 값이나 모순되는 값,
     * 혹은 찾을 수 없는 값이 코드에 전달되는 경우)에 처하게 될 가능성이 있을 때는 코드에 panic!을
     * 넣는 것이 바람직하다.
     *
     * 예를 들어, 아래에 있는 상확 혹은 그 이상일때.
     *
     * "나쁜 상태"는 예기치 못한 무언가이며, 사용자가 입력한 데이터가 잘못된 형식이라던가 하는 흔히
     * 발생할 수 있는 것과는 반대되는 것이다.
     * 그 시점 이후의 코드는 매번 해당 문제에 대한 검사를 하는 것이 아니라, 이 나쁜 상태에 있지
     * 않아야만 할 필요가 있다.
     * 우리가 사용하고 있는 타입 내에 이 정보를 집어넣을만한 뾰족할 수가 없다,
     *
     * 만일 어떤 사람이 우리의 코드를 호출하고 타당하지 않은 값을 집어넣었다면, 가능한 에러를
     * 반환해 라이브러리의 사용자들이 이러한 경우에 대해 어떤 동작을 원하는지 결정할 수 있도록 하는
     * 것이 가장 좋다.
     * 그러나 계속 실행하는 것이 보안상 좋지 않거나 해를 끼치는 경우라면 panic!을 써서 라이브러리를
     * 사용하고 있는 사람에게 자신의 코드에 있는 버그를 알려줘서 개발 중에 이를 고칠 수 있게끔 하는
     * 것이 최선책일 수도 있다.
     * 비슷한 식으로, 우리의 제어권에서 벗어난 외부 코드를 호출하고 있고, 이것이 고칠 방법이 없는
     * 유효하지 않은 상태를 반환한다면, panic!이 종종 적절하다.
     *
     * 실패가 충분히 예상되는 경우라면 panic!을 호출하는 것보다 ,Result를 반환하는 것이 여전히 더
     * 적절하다. 예시로, 잘못된 데이터가 제공된 parser나, 속도 제한에 도달했음을 나타내는 상태를
     * 반환하는 HTTP 요청 등이 있다.
     * 이러한 경우, Result를 반환하면 호출자가 처리 방법을 결정해야 하는 실패 가능성이 예상된다는
     * 것을 나타낸다.
     *
     * 코드가 유효하지 않은 값에 대해 호출되면 사용자를 위험해 빠뜨릴 수 있는 연산을 수행할 때, 그
     * 코드는 해당 값이 유효한지를 먼저 검사하고, 만일 그렇지 않다면 panic!을 호출해야 한다.
     * 이는 주로 보안상의 이유이다.
     * 유효하지 않은 데이터에 어떤 연산을 시도하는 것은 코드를 취약점에 노출시킬 수 있다.
     * 이것이 범위를 벗어난 메모리 접근을 시도했을 경우 표준 라이브러리가 panic!을 호출하는 주된
     * 이유이다.
     * 현재 사용하는 데이터 구조가 소유하지 않은 메모리에 접근 시도하는 것은 흔한 보안 문제이다.
     * 종종 함수에는 입력이 특정 요구사항을 만족시킬 경우에만 함수의 행동이 보장되는
     * 계약(contract)이 있다.
     * 이 계약을 위반했을 때는 패닉을 발생시키는 것이 이치에 맞는데, 그 이유는 계약 위반이 항상
     * Panicking when the contract is violated makes sense because a contract violation always
     * indicates a caller-side bug and it's not a kind of error you want the calling code to have
     * to explicitly handle.
     * There's no reasonable way for calling code to recover; the callling programmers need to fix
     * the code.
     * Contract for a function, especially when a violation will cause a panic, should be explained
     * in the API documentation for the function.
     *
     *
     * However, having lots of error checks in all of your functions would be 
     * verbose(어떤 내용이나 표현이 매우 상세하거나 불필요하게 많은 세부 사항을 포함하고 있는 상태) and annoying.
     * Rust의 타입 시스템과 컴파일러에 의한 타입 체킹은 많은 체크를 대신 해준다.
     * If your code's logic knowing that the compiler has already ensured you have a valid value.
     * For example, if you have a type rather than an Option, your program expects to have
     * something rather than nothing. Your code then doesn't have to handle two cases for the Some
     * and None variants: it will only have one case for definitely having a value. Code trying to
     * pass nothing to your function won't even compile, so your function doesn't have to check for
     * that case at runtime.
     * Another example is using an unsigned integer type such as u32, which ensures the parameter
     * is never negative.
     */

    // 유효성을 위한 커스텀 타입 생성하기:
    // u32 대신 i32로 추릿값을 parsing하여 음수가 입력될 가능성을 허용하고, 그리고서 숫자가 범위
    // 내에 있는지에 대한 검사를 추가적으로 하는 예시가 있다.

    loop {
        // -- 생략--
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
        // --생략
        }
    }

    // 이는 이상적인 해결책이 아니다. 만약 프로그램이 오직 1과 100 사이의 값에서만 동작한다는 점이
    // 굉장히 중요한 사항이고 많은 함수가 동일한 요구사항을 가지고 있다면, 모든 함수 내에서 이런
    // 검사를 하는 것은 지루한 일일 것이다. (게다가 성능에 영향을 줄지도 모른다.)
    // 그대신 새로운 타입을 만들어 그 타입의 인스턴스를 생성하는 함수에서 유효성을 확인하는
    // 방식으로 유효성 확인을 모든 곳에서 반복하지 않게 할 수 있다. 이렇게 하면 함수가 새로운
    // 타입을 시그니처에 사용하여 받은 값을 자신있게 사용할 수 있어 안전하다.
    // 밑의 예제는 new 함수가 1과 100 사이의 값을 받았을 때만 인스턴스를 생성하는 Guess 타입을
    // 정의하는 한 가지 방법을 보여준다.
    
    pub struct Guess {  // 숫자가 저장될 구조체
        value: i32,
    }
    
    impl Guess {
        pub fn new(value: i32) -> Guess {  
            // Guess 값의 인스턴스를 생성하는 new 라는 이름의 associated function을 구현함.
            // associated function은 해당 타입의 인스턴스에 속하지 않고도 호출될 수 있는 함수이다.
            // 인스턴스를 생성하지 않아도 호출할 수 있는 함수로 생성자 구현할 때 사용한다는 뜻임.
            // 주로 해당 타입에 대한 생성자나 팩토리 메서드로 사용됨.
            // 팩토리 메서드는 객체지향 디자인 패턴 중 하나로, 객체를 생성하는 인터페이스를
            // 제공하면서 어떤 클래스의 인스턴스를 생성할지를 서브클래스에서 결정하게 하는
            // 패턴이다. associated function으로 유사한 패턴 구현 가능
            // String::from은 associated function임.
          
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
                // 범위 밖의 value로 Guess를 생성하는 것은 Guess::new가 요구하는 contract를
                // 위반하기 때문이다.
                // Guess::new가 패닉을 일으킬 수 있는 조건은 공개 API 문서에서 다뤄져야 한다.
            }
            Guess { value }
        }

        pub fn value(&self) -> i32 {  
        // getter, value 필드는 비공개이므로 Guess 구조체를 사용하는 코드는 value를 직접 설정할 수 없다.
        // 모듈 밖의 코드는 반드시 Guess::new 함수로 새로운 Guess의 인스턴스를 생성해야 하며,
        // 이를 통해 Guess가 Guess::new의 함수의 조건에 의해 확인되지 않은 value를 가질 수 없음을
        // 보장한다.
            self.value
        }
    }

}
