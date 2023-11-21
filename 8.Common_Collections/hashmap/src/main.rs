use std::collections::HashMap;

fn main() {
    // 해시맵은 파이썬 딕셔너리같은거임!
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 해시맵은 다른 컬렉션보다 사용 빈도가 적어서 prelude에서 자동으로 안 가져옴. 또 표준
    // 라이브러리로부터의 지원을 덜 받음. 
    // 해시맵을 생성하는 기본 제공 매크로가 없음.
    // 해시맵도 데이터를 힙에 저장함.
    // 위의 해시맵은 String type key와 i32 type value 가짐

    // 모든 key끼리, value끼리 같은타입이어야함. 


    // 해시맵의 값 접근하기:
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get 매서드는 Option<&v>를 반환함;
    // 해당 키에 대한 값 없다면 get은 None 반환;
    // 이 프로그램에선 copied를 호출해 Option<&i32> 아닌
    // Option<i32> 얻어온 다음, unwrap_or을 써서 scores가 해당 키에 대한 아이템을 가지고 있지 않을
    // 경우 score에 0을 설정하도록 처리함.
    

    // for 루프를 사용해 해시맵 내의 키/값 쌍에 대한
    // 반복 작업을 수행할 수 있다.

    for (key, value) in &scores {
        println!("{key}: {value}");
    }  // 각각의 쌍은 "임의의" 순서로 출력될 것이다.
    
    // 해시맵과 소유권:
    // i32처럼 copy trait를 구현한 타입의 값은 해시맵 안으로 복사 된다. String 처럼 소유권이 있는
    // 값의 경우, 아래의 예제와 같이 값들이 이동되어 해시맵이 그 값의 소유자가 된다.

    let reason = String::from("love");
    let reality = String::from("Tylenol");
    let mut map = HashMap::new();
    map.insert(reason, reality);
    // HashMap으로 소유권이 이전됨.
    // reasno과 reality는 이 시점부터 유효하지 않다.
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    // This code will print {"Blue": 25}. The original 
    // value of 10 has been overwritten.
    
    // 키가 없을 때만 키와 값 추가하기:
    // 키가 해시맵 내에 존재한다면: 해당 값은 그대로 둔다.
    // 키가 없다면: 키와 그에 대한 값을 추가한다.
    // 해시맵은 이를 위해 entry라고 하는 API를 가지고 있다
    // 검사하려는 키를 매개변수로 받는다.
    // entry 함수의 반환 값은 열거형 Entry인데, 
    // 해당 키가 있는지 혹은 없는지를 나타낸다.

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    
    println!("{:?}", scores);

    let Blue = scores.entry(String::from("Blue")).or_insert(80);
    println!("Blue is: {Blue}");
    // The or_insert method on Entry is defined to return a mutable reference to the value for the
    // corresponding Entry key if that key exists, and if not, inserts the parameter as the new
    // value for this key and returns a mutable reference to the new value. 


    // Updating a Value Based on the Old Value:

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // The split_whitespace method returns an iterator over subslices, separated by whitespace, of
    // the value in text. The or_insert method returns a mutable reference (&mut V) to the value
    // for the specified key. Here we store that mutable reference in the cound variable, so in
    // order to assign to that value, we must first dereference count using the asterisk (*). The
    // mutable reference goes out of scope at the end of the for loop, so all of these changes are
    // safe and allowed by the borrowing rules.




    // Hashing functions:
    // 기본적으로 Hashmap은 해시 테이블과 관련된 서비스 거브 공격에 저항 기능을 제공할 수 있는
    // SipHash라 불리는 해시 함수를 사용한다. 보안에 좋음.
    // 만일 우리 코드를 프로파일링 해보니 기본 해시 함수가 우리의 목적에 사용되기엔 너무 느리다면,
    // 다른 해시어를 지정하여 다른 함수로 바꿀 수 있음.
    // 해시어(hasher)는 BuildHasher 트레이트를 구현한 타입을 말한다. 트레이트와 이를 구현하는
    // 방법에 대해서는 10장에서 자룰 것이다.
    // crates.io에는 수많은 범용적인 해시 알고리즘을 구현한 해시어를 제공하는 공유 라이브러리가
    // 있다.
}
