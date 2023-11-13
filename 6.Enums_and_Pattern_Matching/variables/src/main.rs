fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("Changed value of x is: {x}");
    // 변하지 않을 값은 불변으로 선언하는게 좋다.
    
    // 상수: 항상 불변이다. 값의 타입은 반드시 명시되어야 한다.
    // 상수는 반드시 상수 표현식으로만 설정될 수 있고 런타임에서만 계산될 수 있는 결과값으로는
    // 안된다.
    
    const THREE_HOURS_IN_SECONDS: u32 = 60* 60 * 3;
    // 러스트의 convention은 snake case, 상수는 capital
    // 컴파일러는 컴파일 타임에 제한된 연산을 수행할 수 있는데, 이런 상수값을 10,800으로 스는 대신
    // 이해하고 검사하기 더 쉽게 작성할 방법을 제공해 준다.
    // 상수는 선언된 스코프 내에서 프로그램이 동작하는 전체 시간 동안 유효하다.
}
