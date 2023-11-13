fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // 나머지 모든 가능한 값을 다루는 마지막 갈래에 대한 패턴은 other라는 이름을 가진 변수이다.
        // other 갈래 쪽의 코드는 이 변숫값을 move_player 함수에 넘기는 데 사용한다.
        // 이런 포괄 (Catch-All) 패턴은 match의 철저함을 만족시킨다.
        // 패턴들은 순차적으로 평가되므로 마지막에 포괄적인 갈래를 위치시켜야 한다.
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
        // 포괄 패턴이 필요한데 그 포괄 패턴의 값을 사용할 필요는 없는 경우에 쓸 수 있는 패턴이다.
        // _ 는 어떠한 값이라도 매칭되지만, 그 값을 바인딩하지 않는 특별한 패턴이다.
        // 이는 러스트에게 해당 값을 사용하지 않겠다는 것을 알려주므로, 러스트는 사용되지 않는 
        // 변수에 대한 경고를 띄우지 않을 것이다.
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
        // 마지막으로, 게임의 규칙을 한 번 더 바꿔서 3이나 7 이외의 숫자를 굴리게 되면 아무 일도
        // 일어나지 않도록 해본다.
        // 이는 _ arm에 튜플 절에서 다루었던 유닛 값을 사용하여 표현할 수 있다.
        // 튜플 타입은 고정된 길이를 가지며 한번 선언되면 그 크기를 늘리거나 줄일 수 없다.
        // 아무 값도 없는 튜플은 unit이라는 특별한 이름을 가진다.
        // 이 값과 타입은 모두 ()로 작성되고 빈 값이나 비어있는 반환 타입을 나타낸다.
        // 표현식이 어떠한 값도 반환하지 않는다면 암묵적으로 유닛 값을 반환하게 된다.
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
