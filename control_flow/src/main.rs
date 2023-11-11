fn main() {
    let number = 3;

    if number < 5{  // 코드의 조건은 반드시 bool 이어야 한다.
        println!("condition was true.");  
        // if 식의 조건과 관련된 코드 블럭은 갈래(arms)로 불린다.
    } else{
        println!("condition was false.");
    }

    if number % 4 == 0{
        println!("number is divisible by 4");
    } else if number % 3 == 0{  // else if가 둘 이상일 경우 match란 분기 생성자로 리팩토링 하는게 좋다.
        println!("number is divisible by 3");
    } else{
        println!("well, number isn't divisible by 2 and 3");
    }

    // if가 표현식이기 때문에, let 구문의 우측에 사용할 수 있다.

    let condition = true;

    let number = if condition{
        5
    }else{
        6
    };  
    // if 식의 값은 실행되는 코드 블럭에 따라 다르다.
    // if 식은 expression이므로  if 식에 속한 각 갈래의 결과는 반드시 같은 타입이어야 한다.

    println!("the value of number is: {number}");
}
