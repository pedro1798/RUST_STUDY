fn main() {
    let rect1 = (50, 30);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// rectangle1의 두 개의 파라미터는 연관되어 있지만 어디에도 표현된 바가 없었다.
// 튜플을 이용해 길이와 너비를 함께 묶는다면 더 읽기 쉽고 관리하기도 좋을 것이다.
// 그렇지만 아직 덜 명확하다: 튜플은 요소에 대한 이름이 없어서 튜플 내의 값을 인덱스로 접근해야
// 하기 때문에 계산이 더 혼란스럽다.
