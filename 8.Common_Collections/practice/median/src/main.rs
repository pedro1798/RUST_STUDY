fn main() {
    let mut i = vec![1, 2, 3, 4, 5];  // 정수 리스트가 주어졌을 때 중간값을 출력해 보세요.
    i.sort();
    let median = &i[&i.len() / 2];
    println!("median is: {:?}", median);
    println!("still can use i?: {:?}", &i);
}
