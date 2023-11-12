fn main() {
    // 튜플과 유사하게, 구조체의 구성요소들은 각자 다른 타입을 지닐 수 있다.
    // 튜플과는 다르게 각 구성요소들은 명명할 수 있어 값이 의미하는 바를 명확하게 인지할 수 있다.
    // 위의 이유로 튜플보다 유연하게 다룰 수 있다. 특정 요소 데이터 명세를 기술하거나 접근할 때
    // 순서에 의존할 필요가 없음.
    //
    // 정의한 구조체를 사용하려면, 각 필드의 값을 명세한 인스턴스(instance)를 생성해야 한다.

    let mut user1 = User {
        // key: value로 필드를 정의
        // 필드들의 순서가 정의한 필드의 순서와 같을 필요는 없다.
        email: String::from("peter58@naver.com"),
        username: String::from("pedro17"),
        active: true,
        sign_in_count: 1,
    }; // statement이므로 세미콜론!
    
    user1.email = String::from("exemail@google.com");
    // 변경이 가능한 구조체 인스턴스에 들어있는 값을 바꾸고자 할 때는 점(.) 표기법 사용해 특정
    // 필드에 새 값 할당이 가능하다.

    let user1_email = &user1.email;
    println!("user1_email: {user1_email}");
    drop(user1_email);

    user1.email = String::from("email changed");
    println!("user1.email: {}", user1.email);

    // 인스턴스는 반드시 변경 가능(mutable)해야 한다. Rust에서는 특정 필드만 변경할 수 있도록
    // 허용하지 않는다. 다른 표현식과 마찬가지로, 함수 본문의 마지막에 새 인스턴스 구조체를
    // 표현식으로 생성하여 새 인스턴스를 바로 반환 할 수 있다.

    let user2_email = String::from("user2@google.com");
    let user2_username = String::from("user2");

    let mut user2 = build_user(user2_email, user2_username);
    
    let user3 = User {
        email: String::from("user3@google.com"),
        username: String::from("user3"),
        // 구조체 갱신법 (struct update syntax)는 기존 인스턴스에서 대부분의 값은 재사용하고, 몇몇
        // 값만 바꿔 새로운 인스턴스를 정의하는 문법이다.
        ..user2
        // 입력으로 주어진 인스턴스와 변화하지 않는 필드들을 명시적으로 할당하지 않기 위해 ..구문
        // 사용한다.
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 변수명이 필드명과 같을 때 간단하게 필드 초기화하기:
// 변수명과 구조체의 필드명이 같다면!! 필드 초기화 축약법(field init shortand)을 이용할 수 있다.
fn build_user2(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

/* 구조체 데이터의 소유권(Ownership):
 * User 구조체 정의에서는 &str 문자 슬라이스 타입 대신 String 타입을 사용했다.
 * 이는 의도적인 선택으로, 구조체 전체가 유효한 동안 구조체가 그 대이터를 소유하게 하고자 함이다.
 * 구조체가 소유권이 없는 데이터의 참조를 저장할수는 있지만,
 * 10장에서 언급 될 라이프타임의 사용을 전제로 한다.
 * 라이프타임은 구조체가 존재하는 동안 참조하는 데이터를 계속 존재할 수 있도록 한다.
 * 라이프타임을 사용하지 않고 참조를 저장하고자 하면 에러가 발생한다.
 */

