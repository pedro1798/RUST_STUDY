use std::collections::HashMap;

fn find_mode(numbers: Vec<i32>) -> Option<i32> {
    let mut counts = HashMap::new();
    // 빈도 담을 해시맵

    for &num in &numbers {
        let count = counts.entry(num).or_insert(0);
        // entry는 값이 있으면 Entry 열거형을 반환한다.
        *count += 1
    }

    // 메서드 체이닝의 핵심 아이디어: 각 메서드 호출이 결과로써 객체 자체 또는
    // 변형된 버전을 반환하게 하는 것.

    let (mode, mode_count) = counts
        .iter()  // 해시맵의 이터레이터를 생성한다. 자체로는 순회하지 않음.
        .max_by_key(|&(_, count)| count)  // 메서드가 이터레이터를 사용하여 최댓값을 찾는다.
        // Iterator 트레이트에서 제공되는 메서드.
        // 클로저를 인자로 받아, 클로저의 반환값을 기준으로 최댓값을 찾는다.
        // 이터레이터가 비어있으면 None을 반환한다.
        // value 기준 max를 찾기 위해 key는 와일드카드(_) 처리.
        // None이라면(max가 없다면) None을 반환하고 .map()은 실행되지 않는다.
        // None이 아니라면 Some 반환한다. Option 열거형 반환하니깐!
        .map(|(&num, &count)| (num, count))?;  // 이터레이터를 사용하여 값을 반환한다.
        // ? 문법은 Rust의 에러 핸들링에 사용되는 문법이다.
        // Option의 variant인 None이나 Result의 variant인 Err 라면 해당값을 return하고 함수를
        // 종료한다.
        // 주로 Result나 Option 타입의 값을 처리할 때 사용된다.
        // ?는 Some(value)라면 그 값을 반환하고, None이면 현재 함수에서 바로 None을 반환하며 함수
        // 실행을 종료한다.
        // 주로 Result나 Option을 다룰 때 일련의 작업 중 하나라도 실패하면 전체 함수에서 빠져나가고
        // 싶을 때 유용하다.

    Some(mode)
}

/*
fn find_mode(numbers: Vec<i32>) -> Option<i32> {
    let counts: HashMap<_, _> = numbers.into_iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
            acc
    });
                         
    counts
        .iter()
        .max_by_key(|&(_, count)| count)?
        .0  // Extracting the key from the tuple
}
*/
fn main() {
    let numbers = vec![1, 2, 2, 3, 3, 3, 4, 6];

    if let Some(mode) = find_mode(numbers.clone()){
        // &numbers 대신 numbers.clone을 사용하는 이유:
        // find_mode에서 매개변수로 받은 numbers에 immutable reference를 생성하는데,
        // &numbers로 매개변수를 넘기면 소유권이 없는 reference에 한 번 더 borrow를 하게 되므로
        // 에러 발생함.
        println!("최빈값: {}", mode);
    } else {
        println!("최빈값을 찾을 수 없습니다.");
    }
}
