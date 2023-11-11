fn main() {
    /*
    loop {
        println!("again! ctrl+c to stop.");  // break로 stop 가능
    }
    */
    let mut number = 3;
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while number != 0{
        println!("{number}!");

        number = number - 1;
    }
    println!("LIFTOFF!");

    while index < 5{
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    for element in a.iter(){
        // 코드의 안정성을 높이고 배열의 끝을 넘어가거나 충분한 길이를 지정하지 못해
        // 일부 아이템이 누락되어 발생할 수 있는 버그의 가능성을 제거했다.
        println!("for/ the value is: {}", element);
    }

    for number in (1..4).rev(){
        println!("rev {}!", number);
    }
    println!("LIFTOFF!!!");
}
