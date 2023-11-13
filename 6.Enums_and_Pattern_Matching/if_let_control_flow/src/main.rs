// if let 문법은 if와 let을 조합하여 하나의 패턴만 패칭시키고 나머지 경우는 무시하도록 값을
// 처리하는 간결한  방법을 제공한다.

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximun is configured to be {}", max),
        _ => (),
    }
    // match 표현식을 만족시키려면 딱 하나의 배리언트 처리 후 _ => () 를 붙여야 하는데,
    // 이는 성가신 보일러 플레이트 코드(반복해야 하는 코드)이다.

    // 그 대신, if let을 이용해 이 코드를 더 짧게 쓸 수 있다.

    if let Some(max) = config_max {
        println!("The maximun is configured to be {}", max);
    }
    // if let은 =로 구분된 패턴과 표현식을 입력받는다. 이는 match와 동일한 방식으로 작동하는데,
    // 여기서 표현된 식은 match에 주어지는 것이고 패턴인 이 match의 첫 번째 갈래와 같다.
    // 위의 경우 패턴은 Some(max) 이고, max는 Some 내에 있는 값에 바인딩 된다.
    // if let은 간단하지만 match가 강제했던 철저한 검사를 하지 않는다. 이는 trade off 관계이다.
    // if let은 한 패턴에 매칭될 때만 코드를 실행하고, 다른 경우는 무시하는 match 문을 작성할 때
    // 사용하는 문법 설탕(syntax sugar) 이라고 생각하면 된다.
    
    // if let과 함께 else를 포함시킬 수 있다. else 뒤에 나오는 코드 블록은 match 표현식에서 _
    // 케이스 뒤에 나오는 코드 블록과 동일하다. 
    
    #[derive(Debug)]
    enum UsStates {
        Alaska,
        Alabama,
    }
    enum Coin {
        Penny,
        Quarter(UsStates),
    }    

    let mut count = 0;
    let coin = Coin::Quarter(UsStates::Alabama);
    
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    } // 이 코드는 아래의 if let else 표현식을 사용한 코드와 같다

    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

