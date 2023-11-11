fn main() {

    const MAX_POINT: u32 = 100_000;
    // underscore는 구문적인 요소로 컴파일러에 의해 무시되며 숫자의 값에는 영향을 미치지 않는다.
    println!("MAX_POINT is: {}", MAX_POINT);
    // const naming rule에 따라 대문자 사용
    let mut x = 5;
    // 변수 keyword let은 기본적으로 immutable, mut를 사용해 mutable하게 설정 변경 가능
    // 상수 keyword const는 기본적으로 immutable, mutable하게 바꿀 수 없다. 
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let a = 10;
    println!("a is: {}", a);
    let a = 20;
    println!("shadowed a is: {}", a);
    // let immutable 변수는 shadowing이 가능하다.

    let spaces = "    ";
    let spaces = spaces.len();
    // mut와 shadowing의 차이는 let 키워드를 다시 사용하여 효과적으로 새 변수를 선언하고,
    // 값의 유형을 변경할 수 있으면서도 동일 이름을 사용할 수 있다는 점이다.
    
    /*
    let mut = spaces = "    ";
    spaces = spaces.len();
    */
    
    // 이 코드는 변수의 유형을 변경할 수 없다는 컴파일-시의 에러를 도출한다.
    
    println!("spaces is: {}", spaces);

    //데이터 타입들: 러스트의 타입은 크게 스칼라, 컴파운드의 두 타입으로 나뉜다.

    let guess: u32  = "42".parse().expect("Not a number!");
    // 여기서 type 명시 : u32가 없다면 에러 뱉음

    // 스칼라 타입들: 하나의 값으로 표현되는 타입이다.
    // 정수형, 실수형, boolean, 문자 네 가지의 스칼라 타입을 보유한다.

    let minus: i32 = -29;
    println!("minus is: {}",  minus);

    // 각 부호 변수는 -(2^(n-1)) ~ 2(^(n-1)) - 1 까지의 값을 포함한다.

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    // 기본값인 i32가 일반적으로 좋은 선택이다.
    println!("decimal, hex, octal, binary, byte is: {} {} {} {} {}", decimal, hex, octal, binary, byte);
 
    // 부동 소수점 타입: f32와 f64가 존재. 32bit와 64bit의 크기를 갖는다.
    // 기본 타입은 f64이다. f32와 비슷한 속도를 내면서도 더 정밀한 표현이 가능하기 때문.
    
    let g = 2.0; // f64
    let h: f32 = 3.0;

    // 수학적 연산들:

    let addition  = 5 + 10;
    let subtraction = 95.5 - 4.3;
    let product = 4 * 30;
    let division = 56.7 / 32.2;
    let remainder = 43 % 5;

    // boolean type

    let t = true;
    let f: bool = false;  // with explicit type annotation

    // string type: char이 작은따옴표, String이 큰따옴표
    let c = '😻';
    let z = "z";
    println!("c is: {}", c);

    // char 타입은 Unicode Scalar을 표현하는 값이고 이는 ASCII보다 많은 표현을 가능하게 한다.

    // Compound 타입들: 튜플과 배열

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 튜플은 다양한 타입의 몇 개의 숫자를 집합시켜 하나의 복합 타입으로 만드는 일반적인 방법이다.
    // 우리는 괄호 안에 콤마로 구분되는 값들의 목록을 작성해 튜플을 만든다.
    // 튜플에 포함되는 각 값의 타입이 동일할 필요없이 서로 달라도 된다.
    // 튜플은 단일 요소를 위한 복합계로 고려되었기에 변수 tup에는 튜플 전체가 bind 된다.
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    println!("Indexing tuple: {}, {}, {}", tup.0, tup.1, tup.2);


    // Compound 타입들: 배열

    // 배열의 모든 요소는 모두 같은 타입이여야 합니다. Rust에서 배열은 고정된 길이를 갖는다.
    // 배열의 크기는 커지거나 작아지지 않는다.
    
    let a = [1, 2, 3, 4, 5];

    // 배열은 벡터 타입처럼 가변적이지 않다. 벡터 타입은 확장 혹은 축소가 가능하다. 
    // 뭘 사용할지 확실하지 않은 상황이라면 벡터를 사용하도록 하자.

    // 배열은 stack에 단일 메모리 뭉치로 할당된다. index를 통해 배열의 요소에 접근할 수 있다.

    let first = a[0];
    let second = a[1];

    // index가 배열의 길이보다 길면 rust는 panic한다.
}
