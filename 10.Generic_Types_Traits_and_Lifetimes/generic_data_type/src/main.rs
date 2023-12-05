fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
// largest 함수는 어떤 타입 T에 대한 제네릭 함수

/* 함수 본문에서 T 타입 값들에 대한 비교가 필요하므로, 여기에는 값을 정렬할 수 있는
 * 타입에 대해서만 동작할 수 있다.
 * 비교가 가능하도록 하기 위해, 표준 라이브러리는 임의의 타입에 대해 구현 가능한
 * std::cmp::PartialOrd 트레이트를 제공한다.
 * T: std::cmp::PartialOrd는 T가 PartialOrd를 구현한 것일 때만 유효하도록 제한을 두는 코드이다.
 * 표준 라이브러리가 i32와 char 둘 모두에 대한 PartialOrd를 구현하고 있다.
 */
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,  
    // x와 y는 T로 동일한 타입이어야함.
}

struct Point2<T, U> {
    x: T,
    y: U,  
    // 여러 개의 제네릭 타입 매개변수를 사용해 x는 T 타입으로, y는 U 타입으로 정의할 수 있음.
    // 제네릭 타입 매개변수는 많아질수록 코드 가독성은 떨어진다.

}

fn main() {
    // 제네릭 함수를 정의할 때는, 함수 시그니처 내 매개변수와 
    // 반환 값의 데이터 타입 위치에 제네릭을 사용한다.
    
    // 새 단일 함수의 시그니처 내 타입읠 매개변수화 하려면
    // 타입 매개변수의 이름을 지어줄 필요가 있다.
    // 러스트에서는 타입 이름을 지어줄 때는 대문자로 시작하는 낙타 표기법(UpperCamelCase)를 따르고,
    // 타입 매개변수의 이름은 짧게(종종 한 글자로만) 짓는 것이 관례이므로, type을 줄인 T를 사용한다.
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest int is: {}", result);

    let char_list = vec!['y', 'n', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is: {}", result);
    
    let integer = Point { x: 5, y: 10};
    let char = Point {x: 1.0, y: 4.0};

    /* 제네릭 열거형 정의
     * 구조체처럼, 열거형도 배리언트에 제네릭 데이터 타입을 갖도록 정의할 수 있다.
     */

    /* enum Option<T> {
     *   Some(T),
     *   None,
     * }
     *
     * Option<T> 열거형은 T 타입에 대한 제네릭이며, T 타입을 들고 있는 Some 배리언트와 아무런
     * 값도 들고 있지 않은 None 배리언트를 갖는다. 옵션 값이 어떤 타입이건 상관없이 추상화하여
     * 사용할 수 있다. 열거형에서도 여러 개의 제네릭 타입을 이용할 수 있다. 
     *
     * enum Result<T, E> {
     *     Ok(T),
     *     Err(E),
     * }
     *
     * 작성한 코드에서 보유하는 값의 타입만 다른 구조체나 열거형이 여러 개 있음을
     * 발견했을 때는 제네릭 타입을 사용해 코드 중복을 제거할 수 있다.
     */
}
