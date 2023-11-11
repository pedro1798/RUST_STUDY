fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    // &는 참조자 이며, 어떤 값을 소유권을 넘기지 않고 참조할 수 있도록 해준다.
    println!("The length of {s1} is {len}.");
    // & 문법은 소유권 갖고 있지 않기 때문에, 이 참조자가 가리키는 값은 참조자가 
    // 스코프 밖으로 벗어났을 때도 메모리가 반납되지 않는다.
    // 함수의 파라미터로 참조자를 만드는 것을 borrowing 이라고 한다.
    
    change(&mut s1);
    println!("s1 is: {s1}");

    // 가변 참조자는 큰 제한이 있다:
    // 특정한 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있다.
    // 다음과 같은 코드는 실패한다.

    let mut m = String::from("abcd");
    let m1 = &mut m;
    
    // let m2 = &mut m; 
    // cannot borrow 'm' as mutable than once at a time.
    // Rust가 compile time의 data race를 방지해 준다.
    // Data Race가 나타나는 조건:
    // 1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
    // 2. 그 중 적어도 하나의 포인터가 데이터를 쓴다.
    // 3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.
    // 러스트는 데이터 레이스가 발생할 수 있는 코드는 컴파일조차 할 수 없게 막는다.

    //println!("m is: {m}, m1 is: {m1}");

    // borrow 상태에서 원래의 요소 사용 못 함??
    // println!로 인자를 넘길 때 immutable 취급인데 mutable reference 있어서 안됨?
    // when you use a variable or expression within curly braces of 'println!', it is used in an
    // immutable way. The contents of the variable or expression are effecrively copied or borrowed
    // for the purpose of printing, and the original variable or expression remains unchanged.
    // m will use the Display trait to format the value of 'a' for printing. but it won't consume
    // or mutate the original 'a'. it's essentially a read-only operation for the purpose of
    // printing.
    
    let mut r = String::from("abraca-dabra");
    let r1 = &r;  // 불변 참조자로 여러 번 참조 가능
    let r2 = &r;  // 위와 같음
    // let r3 = &mut r; // 큰 문제

    // cannot borrow 'r' as mutable because it is also borrowed as immutable.
    // 불변 참조자를 가지고 있을 동안에도 역시 가변 참조자를 만들 수 없다.
    // 불변 참조자의 사용자는 사용중인 동안에 값이 갑자기 바뀌리라 예상하지 않는다. 
    // 하지만 여러 개의 불변 참조자는 만들 수 있는데, 데이터를 그냥 읽기만하는 것은 다른 것들이 그
    // 데이터를 읽는데에 어떠한 영향도 주지 못하기 때문이다.
    
    let mut a = String::from("hello");
    println!("a is: {a}");
    println!("a again is: {a}");
}

fn calculate_length(s: &String) -> usize{
    s.len()  // 팁: expression이기 때문에 세미콜론 안붙임
}

fn change(some_string: &mut String){
     some_string.push_str(" this won't work if you don't use mut.");
    // cannot borrow immutable content ad mutable
    // 변수가 기본적으로 불변인 것 처럼 (let)
    // 참조하는 어떤 것을 변경하는 것은 허용되지 않는다.
}

