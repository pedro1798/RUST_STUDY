#[derive(Debug)]  // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter variant has UsState value
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 매치 갈래의 또 다른 유용한 기능은 패턴과 매칭된 값들의 일부분을 바인딩할 수 있다는
        // 이것이 열거형의 variant로부터 어떤 값들을 추출할 수 있는 방법이다.
        // 한 가지 예로, 열거형 배리언트 중 하나가 내부에 값을 들고 있다.
        // 매치 표현식 내에는 variant Coin::Quarter의 값과 매칭되는 pattern에 state라는 
        // 이름의 변수를 추가한다. Coin::Quarter이 매치될 때, state 변수는 그 쿼터 동전의
        // 주에 대한 값에 바인딩될 것이다.
        // 그러면 우리는 다음과 같이 해당 갈래에서의 코드 내에서 state를 사용할 수 있다.

        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        }
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alabama);
    let v = value_in_cents(c);
    println!("v is: {}", v);
}
