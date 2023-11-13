fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // Some 내부에 담긴 값은 i에 바인딩되므로, i는 값 5를 갖는다.
        // 그런 다음 매치 갈래 내의 코드가 실행되므로, i의 값에 1을 더한 다음
        // 최종적으로 6을 담은 새로운 Some 값을 생성한다.
    }
    // match와 열거형을 조합하는 것은 다양한 경우에 유용하다.
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
