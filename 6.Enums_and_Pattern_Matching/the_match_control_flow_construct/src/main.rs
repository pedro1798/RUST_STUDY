enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {  
        // match 키워드 뒤 표현식. if의 경우 조건문에서 부울린 값을 반환해야 하지만, 여기선
        //어떤 타입이든 가능하다.
        
        Coin::Penny => 1,  // match의 갈래(arm)들이다. 하나의 갈래는 패턴과 코드 두 부분으로 이루어져 있다.
        Coin::Nickel => 5,  // 각 갈래와 연관된 코드는 표현식이고, 매칭 갈래에서의 표현식의 결과로써
        Coin::Dime => 10,  // 생기는 값은 전체 match 표현식에 대해 반환되는 값이다.
        Coin::Quarter => {
            println!("Most expensive coin!");
            25
        },
    }
}

fn main() {
    // 러스트는 match 라고 불리는 강력한 제어 흐름 연산자를 가지고 있다.
    // 이는 일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매칭되었는지를 바탕으로 
    // 코트를 수행하도록 해준다.
    // 패턴은 리터럴 값, 변수명, 와일드카드 등 다양한 것으로 구성될 수 있다.
    // match의 힘은 패턴의 표현성으로부터 오며 컴파일러는 모든 가능한 경우가 처리되는지 검사한다.
    // 동전 분류기를 상상해 보세요!
}
