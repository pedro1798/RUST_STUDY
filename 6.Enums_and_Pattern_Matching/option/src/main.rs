fn main() {
    // Option 타입은 값이 있거나 없을 수 있는 아주 흔한 상황을 나타낸다.
    // 비어있는 리스트, 비어있지 않은 리스트 등 과 같은
    // 이 개념을 타입 시스템으로 표현한다는 것은 처리해야 하는 모든 경우를 처리했는지 컴파일러가
    // 확인 할 수 있다는 의미이다.
    // 러스트는 다른 언어들에서 흔하게 볼 수 있는 null 개념이 없다.
    // null의 문제점: 널 값을 널이 아닌 값처럼 사용하려고 할 때 여러 종류의 에러가 발생할 수 있다.
    // 현재 어떠한 이유로 인해 유효하지 않거나, 존재하지 않는 하나의 값 이라는 널이 표현하려는
    // 개념은 여전히 유용하다.
    // 널의 문제는 개념에 있기보단 구현에 있다. 
    // 러스트에는 널이 없지만, 값의 존재 혹은 부재의 개념을 표현할 수 있는 열거형이 있다
    // 그 열거형이 바로 Option<T> 이며, 다음과 같이 표준 라이브러리에 정의되어 있다.
    
    let some_number = Some(5);  // 타입은 Option<i32>
    let some_char = Some('e');  // 타입은 Option<char>

    let absent_number: Option<i32> = None;  
    // 전반적인 Option 타입을 명시해야 한다. None 값만 봐서는 동반되는 Some variant가 어떤 타입의
    // 값을 가질지 컴파일러가 추론할 수 없기 때문이다.
    // Some 값을 얻게 되면, 값이 존재한다는 것과 해당 값이 some 내에 있다는 것을 알 수 있다.
    // None 값을 얻게 되면, 얻은 값이 유효하지 않다는, 어떤 면에선 널과 같은 의미를 가진다.
    // Option<T>가 널보다 나은 이유는??
    // Option<T>와 T는 다른 타입이기 때문에, 컴파일러는 Option<T> 값을 명백하게 유효한 값처럼
    // 사용하지 못하도록 한다.

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
    // 이 코드를 실행하면 아래의 에러 메시지가 출력된다.
    // cannot add 'Option<i8>' to 'i8'
    // the trait 'Add<Option<i8>>' is not implemented for 'i8'

    // Option을 사용하면, 값이 있을지 없을지에 대해 걱정할 필요가 있으며, 컴파일러는 값을 사용하기
    // 전에 이런 경우가 처리되었는지 확인해 준다.
    // 바꿔 말하면, T에 대한 연산을 수행하기 전에 Option<T>를 T로 변환해야 한다.
    // 이런 방식은 널로 인해 발생하는 가장 흔한 문제인, 실제로는 널인데 널이 아니라고 가정하는
    // 상황을 발견하는 데 도움이 된다.
    // 널일 수 있는 값을 사용하기 위해서는 명시적으로 값의 타입을 Option<T>로 변환해야 한다.
    // 그 다음 값을 사용할 때 명시적으로 널인 경우를 처리해야 한다.
    // Option<T>에서 값을 가져오는 법은 직접 찾아보기.
    
    let x: Option<i32> = None;
    // assert_eq! 매크로는 두 값이 동일한지 체크한다. 동일하지 않다면 패닉에 빠진다.
    assert_eq!(x.is_some()), false);  // is_some: Returns ture if the option is a Some value.
}

enum Option<T> {
    None,
    Some(T),
}

// Option 열거형은 너무나 유용하기 때문에, 러스트에서 기본으로 임포트하는 목록인 prelude에 포함됨.
// 이것의 배리언트 또한 프렐루드에 포함됨.
// 그래서 Some, None 배리언트 앞에 Option::도 붙이지 않아도 됨.
