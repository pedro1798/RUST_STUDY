fn main() {
    let x = 5;
    let x = 10; // shadowing
    {
        let x = x * 2;
        println!("The value of in the inner scope is: {x}");
    }  // 이 스코프가 끝나면 안쪽의 섀도잉은 끝나서 x는 다시 10으로 돌아온다.
    println!("The value of x is: {x}");
    // let을 사용하면, 값을 변형하면서 변형이 완료된 후에는 불변으로 유지할 수 있다.
}
