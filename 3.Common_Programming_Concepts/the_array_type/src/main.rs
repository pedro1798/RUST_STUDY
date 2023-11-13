fn main() {
    // 배열의 모든 요소는 모두 같은 타입이며 배열은 고정된 길이를 가진다.
    
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    // 다음과 같이 대괄호 안에 요소의 타입을 스고 세미콜론을 쓴 뒤 요소의 개수를 적는 식으로 배열의
    // 타입을 작성할 수도 있다.

    let a = ["great "; 5];
    let visited = [0; 10];

    // 다음과 같이 대괄호 안에 초깃값과 세미콜론을 쓴 다음 배열의 길이를 적는 방식을 사용하여 모든
    // 요소가 동일한 값으로 채워진 배열을 초기화할 수도 있다.
    
    println!("a is: {:?}", a);

    for element in &a {
        println!("a is: {}", element);
    }

    println!("visited 3rd element is: {}", &visited[2]);
    
}
