fn main() {
    let v: Vec<i32> = Vec::new();
    // 벡터를 사용하면 메모리에서 모든 값을 서로 이웃하도록 배치하는 단일 데이터 구조에 하나 이상의
    // 값을 저장할 수 있다. 
    // 벡터는 같은 타입의 값만을 저장할 수 있다.
    // 벡터는 파일 내의 텍스트 라인들이나 장바구니의 품목 가격 같은 아이템 목록을 저장하는 상황일
    // 대 유용하다.
    // 벡터는 제네릭을 이용해 구현됐다.
    // 대부분의 경우는 initail values와 함께 벡터를 만들고 rust는 infer(추론) 할 수 있기 때문에, 
    // <i32>와 같은 type annotation은 쓸 일이 별로 없다.
    // 러스트는 conventionally vec! 매크로를 지원한다.
    let v = vec![1, 2, 3];
    // the integer types is i32 because that's the default type.
    // Rust can infer that the type of v is Vec<i32>, and type annotation isn't necessary.

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);
    // v.push("panic!"); rust infers type of vector as i32, pushing str type occurs error.
    
    // Reading Elements of Vector

    let v = vec![1, 2, 3, 4];
    let third: &i32 = &v[2]; // indexing
    // &와 []를 사용하면 인덱스 값에 위치한 요소의 참조자를 얻게 된다.
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);  // get method
    // get 함수에 인덱스를 매개변수로 넘기면, match를 통해 처리할 수 있는 Option<&T> 를 얻게 된다.
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
    // 벡터에 없는 인덱스 값을 사용하고자 했을 때 프로그램이 어떻게 동작할 것인지 선택할 수 있도록
    // 하기 위해서이다.
    
    let v = vec![1, 2, 3, 4, 5];
    let does_not_exis: &i32 = &v[100];  // 존재하지 않는 요소를 참조하기 때문에 패닉을 일으킴.
    // 실험해보니 panic 안 일으키고 index out of bounds: ... 출력함

    let a = vec![1, 2, 3];
    println!("{:?}", a[4]);

    // 컴파일러는 expression의 값이 아닌 type을 따진다. 그러므로
    // a 는 Vec<i32> 타입이다.
    // 4 는 unknown integral type이다.
    // *In this context integral means "of or denoted by an integer", not "built-in".
    // Vec<i32> implements subscripting, so a[4] type checks
    // subscripting은 일반적으로 대괄호를 사용하여 배열, 슬라이스 또는 기타 데이터 구조에서 요소에
    // 접근하는 작업을 나타낸다.
    // vector no longer carries length information, its length is only known at runtime.

    let does_not_exist = v.get(100);  // 패닉 없이 None이 반환됨. None 처리하는 로직 있어야 한다.

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];  // immutable reference borrow가 있다면
    v.push(6);
    println!("The first element is: {first}");
    // cannot borrow 'v' as mutable because it is also borrowed as immutable 에러 발생한다.
    // 벡터의 동작 방법:
    // 벡터는 모든 요소가 서로 붙어서 메모리에 저장된다. 그리고 새로운 요소를 벡터 끝에 추가할
    // 경우, 현재 벡터 메모리 위치에 새로운 요소를 추가할 공간이 없다면, 다른 넉넉한 곳에 메모리를
    // 새로 할당하고 기존 요소를 새로 할당한 공간에 복사한다. 이 경우, 기존 요소의 참조자는 해제한
    // 메모리를 가리키게 되기 때문에, 이러한 상황을 대여 규칙으로 막아둔 것이다.

    // 벡터 값에 대해 반복하기:
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;  
        // 가변 참조자가 가리키는 값을 수정하려면 += 연산자를 쓰기 전에 * dereference 연산자로 i값을 얻어야 한다.
        // 벡터에 대한 반복 처리는 불변이든 가변이든 상관없이 대여 검사 규칙에 의해 안전하다.
        // 위의 두 예제의 for 루트 본문에서 벡터에 아이템을 추가하거나 지우는 시도를 했다면 컴파일
        // 에러가 발생한다. for 루트가 가지고 있는 벡터에 대한 참조자는 전체 벡터에의 동시다발적
        // 수정을 막는다.
    }

    // 열거형을 이용해 여러 타입 저장하기::
    // 벡터는 같은 타입을 가진 값들만 저장할 수 있다. 다른 타입의 아이템들에 대한 리스트를 저장해야
    // 하는 상황이 있다면: 열거형의 배리언트는 같은 열거형 타입 내에 정의가 되므로, 벡터 내에 다른
    // 타입의 값들을 저장할 필요가 있다면 열거형을 정의하여 사용할 수 있다.
    // Capsulization 같은건가?

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ]; // statement니까 세-마이-콜론 달기^^ 
    // 러스트가 컴파일 타임에 벡터 내에 저장될 타입이 무엇인지 알아야 하는 이유는 각 요소를
    // 저장학ㅣ위해 얼마만큼의 힙 메모리가 필요한지 알아야 하기 때문이다. 또한 이 벡터가 담을 수
    // 있는 타입을 명시적으로 보여줘야 한다. 만일 러스트가 어떠한 타입이든 담을 수 있는 벡터를
    // 허용한다면, 벡터의 각 요소마다 수행되는 연산에 대해 하나 혹은 그 이상의 타입이 에러를
    // 발생시킬 수 있다. 열거형과 match 표현식을 사용한다는 것은 6장에서 설명한 것처럼 러스트가
    // 컴파일 타임에 가능한 모든 경우를 처리함을 보장해 준다는 뜻이다.

    // 런타임에 프로그램이 벡터에 ㅓ장할 모든 타입 집합을 알지 못하면 열거형을 이용한 방식은 사용할
    // 수 없을 것이다. 대신 trait object를 이용할 수 있는데, 17장에서 다룰 예정이다.

}  // 여기서 v가 스코프 밖으로 벗어나고 해제된다.
// 벡터는 스코프 밖으로 벗어날 때 내용물 전부 버려진다. 벡터가 가지고 있던 정수들의 메모리도
// 정리된다.
// 대여 검사기는 벡터의 내용물에 대한 참조자의 사용이 해당 벡터가 유효할 때만 발생했는지 확인한다.

