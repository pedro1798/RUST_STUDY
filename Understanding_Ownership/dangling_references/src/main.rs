fn main() {
    /*
    댕글링  포인터란 어떤 메모리를 가리키는 포인터를 보존하는 동안,
    그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을 지도 모를 메모리를
    참조하고 있는 포인터를 말한다.
    */

    /*
    러스트에선 컴파일러가 모든 참조자들이 댕글링 참조자가 되지 않도록 보장한다.
    우리가 어떤 데이터의 참조자를 만들었다면, 컴파일러는 그 참조자가 스코프 밖으로
    벗어나기 전에는 데이터가 스코프 밖으로 벗어나지 않을 것임을 확인해 준다.
    */

    // 아래는 댕글링 참조자를 만드는 예시이다. 오류가 발생한다.

    // let reference_to_nothing = dangle();
    let no_d = no_dangle();
    println!("no_d is: {no_d}");
}

fn dangle() -> &String {
    let s = String::from("Hello?");
    &s  

    // 참조자가 스코프 밖으로 return된다.
    // lifetime은 10장에서 자세하게 다룰 것이다.
    // this function's return type contains a borrowed value, but there is no value for
    // it to be borrowed from.
    
    // &s는 s를 참조하는데, dangle의 코드가 끝나면 s는 할당 해제(drop)된다.
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // 해결법은 String을 직접 반환하는 것이다.
    // 소유권이 밖으로 이동되었고, 아무것도 할당 해제되지 않는다.
}
/*
참조자의 규칙:
1. 어떠한 경우이든 간에, 아래의 둘 중 하나만 가질 수 있다.
- 하나의 가변 참조자
- 임의 개수의 불변 참조자들
2. 참조자는 항상 유효해야만 한다.
*/
