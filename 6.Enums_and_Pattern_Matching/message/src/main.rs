enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Message엔 다른 데이터 타입을 갖는 네 개의 배리언트가 있다.
// Quit은 연관덴 데이터가 없다. Move는 구조체처럼 이름이 있는 필드를 갖는다. 이하 ..
// 모든 베리언트가 Message 타입으로 묶인다.

// 구조체와 비슷하게 impl을 사용해 메서드를 정의할 수 있다.

impl Message {
    fn call(&self) {
        println!("I'm calling to...");
    }
}

fn main() {
    let m = Message::Write(String::from("choi"));
    m.call();
}
