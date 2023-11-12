#[derive(Debug)]  // 이렇게 Debug 어노테이션을 추가한다. 파생 트레잇 derived trait
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    // 프로그램을 디버깅하는 동간 구조체 내의 모든 값을 보기 위해서 Rectangle의 인스턴스를
    // 출력할 수 있다면 도움이 될 것이다.
    
    let rect1 = Rectangle { length: 50, width: 30, };
    // println!("rect is {}", rect1);

    // println! 매크로는 다양한 종류의 포맷을 출력할 수 있으며, 기본적으로 {}은 println! 에게
    // Display라고 알려진 포맷딩을 이용하라고 전달해준다: 직접적인 최종 사용자가 사용하도록 의도된
    // 연출이다. 지금까지 우리가 봐온 기본 타입들은 Display가 기본적으로 구현되어 있는데, 이는 1
    // 혹은 다른 기본 타입을 유저에게 보여주고자 하는 방법이 딱 한가지이기 때문이다.

    // 하지만 구조체를 사용하는 경우, println!이 출력을 형식화하는 방법은 덜 명확하다. 표시 방법의
    // 가능성이 많기 때문이다. 어느 필드를, 구분자는 띄어쓰기? 쉼포? 중괄호를 출력/출력하지 않음?
    // 이러한 모호성 때문에 러스트는 우리가 원하는 것을 추론하려는 시도를 하지 않으며
    // 구조체는 Display에 대한 기본 제공 되는 구현체를 가지고 있지 않다.

    println!("rect is {:?}", rect1);
    // :? specifier (명시자)를 집어넣는 것은 pringln!에게 Debug라 불리는 출력 포맷을 사용하고
    // 싶다고 말하는 것이다. Debug는 개발자에게 유용한 방식으로 우리의 구조체를 출력할 수 있도록
    // 해줘서 우리 코드를 디버깅 하는 동안 그 값을 볼 수 있게 해주는 트레잇이다.
    // 하지만 여전히 버그가 발생한다:
    // error: the trait bound 'Rectangle: std::fmt::Debug' is not satisfied

    // 우리 구조체에 대하여 디버깅 정보를 출력하는 기능을 활성화하도록 명시적인 사전동의를 해줘야
    // 한다. 그러기 위해서, 구조체 정의부분 바로 전에 #[derive(Debug)] 어노테이션(The Book
    // 한글판에선 어노테이션, 영어판에선 Attribute임! Attribute라고 하기.) 을 추가한다.
    
    // 좀 더 예쁜 출력을 원한다면?
    println!("rect is {:#?}", rect1);
}
