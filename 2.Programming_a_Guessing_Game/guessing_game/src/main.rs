extern crate rand;

use std::io;
use std::cmp::Ordering;  // comparison
use rand::Rng;  // Random Number Generator

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);
    
    loop{
        println!("Please input your guess.");

        let mut guess = String::new();
        // let은 변수 생성 예약어 
        // mut이 있다면 mutable, 없으면 immutable

        io::stdin().read_line(&mut guess)
	        .expect("Fail to read line");
   
        let guess: u32 = match guess.trim().parse(){
            // parse가 성공적으로 문자열에서 정수로 변환했다면 결과값을 가진 Ok를 돌려준다.
            Ok(number) => {
                println!("number is: {}", number);
                number
            },
            Err(_) => {
                println!("You need to input Integer!");
                continue;
            },
            // _ (언더스코어)는 wildcard로 모든 값과 매칭될 수 있음
            // 효율적인 프로그램은 parse에서 가능한 모든 에러를 무시함.
        };
        // : u32는 type 명시 (unsigned 32bit integer)
        // parse()는String type을 type에 맞는 정수로 parse

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

// i32는 32비트 정수, u32는 32비트의 부호없는 정수, i64는 64비트의 정수, ...
// 명시 없으면 숫자들은 i32로 생각함
