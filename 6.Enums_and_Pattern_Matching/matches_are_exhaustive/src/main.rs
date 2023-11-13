fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
    // 여기선 None 케이스를 다루지 않았고, 따라서 이 코드는 버그를 일으킬 것이다.
    // 러스트의 매치는 철저하다. 발생할 수 있는 경우 중 놓친 게 있음을 아는 것은 물론, 
    // 어떤 패턴을 놓쳤는가도 알고 있다.
    // 유효한 코드를 만들려면 모든 가능성을 샅샅이 다루어야 한다.
}

fn main() {
    println!("Hello, world!");
}
