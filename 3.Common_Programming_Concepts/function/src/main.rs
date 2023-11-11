fn main() {
    println!("Hello, world!");
    let y = {
        let x = 3;
        x + 1// x + 1은 4값을 return해 x에 할당하는 expression이다. bracket 또한 expression임.
             // expressions do not include ending semicolons. expressions는 세미콜론 안 붙인다.
             // expression에 ; 붙이면 expression을 statement로 바꾸고 따라서  value를 return하지 않는다.
    };
    // println!("x must be 4: {x}"); 이 스코프에선 위에서 선언된 x 찾을 수 없음

    println!("The value of y is: {}", y);  // 4

    let x = another_function(5);
    println!("The return value of function is: {x}");
}

fn another_function(x: i32) -> i32{  // 러스트는 함수의 위치를 신경쓰지 않는다.
    println!("The value of x is: {}.", x);
    10  // In Rust, the return value of the function is synonymous with the value of the final expression
       // in the block of the body of a function.
       // You can return early from a function by using the return keyword and specifying a value,
       // but most functions return the last expressin implicitly.
       // returns 5
}

// 러스트의 함수엔 두 가지 요소가 있다: statements와 expressions. statements는 retrun값 없음.
// C에서 x = y = 6; 은 x와 y에 6을 할당하지만 러스트에선 가능하지 않다. 할당은 statement이며 리턴값 없으므로.
// statement의 예: let x = 5;
// function definition 또한 statement이다.

// expression의 예시엔 math operation 등이 있다. 예) 5 + 6 에서 +는 evaluates to the value 11

// expression can be part of statements. let x = 6; 에서 6은 6이란 값을 산출하는 expression.
// calling a function is an expression. calling a macro is an rxpression.
// a new scope block created with curly brackets is an expression.


