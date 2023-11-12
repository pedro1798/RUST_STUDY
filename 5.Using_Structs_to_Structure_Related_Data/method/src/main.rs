/*
 * 메소드는 함수와 유사하다. fn 키워드로 선언되고, 파라미터와 반환값을 가지며, 다른 어딘가로부터
 * 호출되었을때 실행될 코드를 담고 있다. 하지만, 메소드는 구조체의 내용 안에서 정의되며(혹은
 * 열거형이나 트레잇 객체 안에 정의되는데), 첫번째 파라미터가 언제나 self인데 이는 메소드가
 * 호출되고 있는 구조체의 인스턴스를 나타낸다.
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {  
    // Rectangle 구조체 상에 area 메소드 정의하기위해서impl(implementation) 블록을 시작한다.

    fn area(&self) -> u32 {
    // area의 시그니처 내에서는 rectangle: &Rectangle 대신 &self가 사용되었는데 이는 메소드가 impl
    // Rectangle 내용물 안에 위치하고 있어 러스트가 self의 타입이 Rectangle라는 사실을 알 수 있기
    // 때문이다. 
    // self에도 여전히 &가 붙어 있음을 주목하자. 메소드는 self의 소유권을 가져갈 수도, self를 변경
    // 불가능하게 빌릴 수도, 다른 파라미터와 비슷하게 변경이 가능하도록 빌려올 수도 있다.
    // 그냥 self 파라미터로 인스턴스의 소유권을 가져오는 메소드를 작성하는 일은 드물다; 
    // 이러한 테크닉은 보통 해당 메소드가 self를 다른 무언가로 변형시키고 이 변형 이후에는 호출하는
    // 측에서 원본 인스턴스를 사용하는 것을 막고 싶을 때 종종 쓰인다.
    // 메소드를 함수 대신 사용하는 주된 이유는 코드 조직화(organization)에 관한 점이다.
    // 우리 코드를 향후 사용할 사람들이 우리가 제공하는 라이브러리 내의 다양한 곳에서 Rectangle이
    // 사용 가능한 지점을 찾도록 하는 것보다 하나의 impl 블록 내에 해당 타입의 인스턴스로 할 수
    // 있는 것을 모두 모와두었다.
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    // 연관 함수: parameter로 &self를 받지 않다면 이는 연관 함수(associated functions)으로,
    // 메소드가 아닌 함수이다. 그럼에도 impl 내에 정의하는 이유는 이 함수가 해당 구조체와 연관되어
    // 있기 때문이다. 함수인 이유는 함께 동작할 구조체의 인스턴스를 가지고 있지 않아서이다.
    // 연관 함수는 새로운 구조체의 인스턴스를 반환해주는 생성자로서 자주 사용된다.
    // 아래는 하나의 차원값 파라미터를 받아서 이를 길이와 너비 양쪽에 사용하여, 정사각형
    // Rectangle을 생성할 때 같은 값을 두 번 명시하도록 하는 것 보다 쉽게 해주는 연관 함수를 제공할
    // 수 있다.
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30, };
    let rect2 = Rectangle { length: 40, width: 20, };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    let sq = Rectangle::square(10);
    // 이처럼 연관 함수를 호출하기 위해서는 구조체 이름과 함께 :: 문법을 이용한다. 이 함수는
    // 구조체의 namespace 내에 있다. :: 문법은 연관 함수와 모듈에 의해 생성된 namespace 두 곳
    // 모두에서 사용되는데, 모듈에 대해서는 7장에서 다룬다.
}
