//main 내 같은 기능을 하는 코드뭉치를 함수로 정의하여 추상화했다.
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > list {
            largest = item;
        }
    }
    largest
}

// 중복되는 코드 찾아서 함수로 추상화하는 예제

fn main() {
    /* Generic allow us to replaca specific types with a placeholder that represents
     * multiple types to remove code duplication.
    */
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];
    // number은 &i32 type이므로 number과 largest의 직접적인 비교 위해&number_list[0]함.
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("{:?}\nlargest: {}", number_list, largest);
}
