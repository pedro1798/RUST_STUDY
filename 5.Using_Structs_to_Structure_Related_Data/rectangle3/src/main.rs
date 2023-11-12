struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30, };
    // main의 rect1은 구조체의 소유권은 유지하며 그러므로 rect1을 계속 이용할 수 있다.
    // 이는 우리가 함수 시그니처 내에서와 함수 호출시에 &를 사용하게 된 이유이다.
    println!("The area of the rectangle is {} sqare pixels.", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
    // 인덱스가 아님 필드의 이름으로 접근해 더욱 의도가 명확해졌다.
}
