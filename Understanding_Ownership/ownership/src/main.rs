fn main() {
    // 스택은 새로운 데이터를 넣어두기 위한 공간 혹은 데이터를 가져올 공간을 검색할 필요 없음
    // 항상 스택의 꼭대기이기 때문에.
    // 스택에 담긴 모든 데이터는 결정되어 있는 고정된 크기를 갖고 있어야 한다.
    // 컴파일 타임에 크기가 결정되어 있지 않거나 크기가 변경될 수 있는 데이터를 위해선
    // 힙에 데이터를 저장할 수 있다.
    // 힙에 데이터를 넣을 때 저장할 공간이 있는지 물어보고 그러면 운영체제가 충분히 컬다란 힙 안의
    // 빈 지점을 찾아서 이 곳을 사용중이라고 표시하고, 해당 지점의 포인터를 돌려준다
    // 이를 allocating이라 부른다. 스택은 allocating 아님.
    // 힙에 저장된 데이터는 포인터가 가리킨 곳을 따라가야 하기 때문에 스택보다 느리다.
    let s = "hello!";
    // "hello"는 스트링 리터럴. 변수는 선언된 시점부터 현재의 스코프가 끝날 때까지 유효하다.
    // 스트링 리터럴의 값은 우리의 프로그램의 텍스트 내에 하드코딩되어있다.
    // 이전에 봐온 모든 데이터 타입들은 스택에 저장되었다가 스코프를 벗어날 때 스택으로부터 팝된다.
    // 이제 힙에 저장되는 데이터를 관찰하자.
    // 스트링 리터럴은 immutable하다.
    // String은 힙에 할당되며 컴파일 타임에는 우리가 알 수 없는 양의 텍스트를 저장할 수 있다.

    let mut s = String::from("hello");

    // ::은 String 타입 아래 from 함수를 특정지을 수 있도록 해주는 네임스페이스 연산자이다.
    
    s.push_str(", world!"); // push_str은 해당 스트링 리터럴을 스트링에 붙여준다.
    println!("{s}");

    // 스트링 리터럴은 텍스트가 최종 실행파일에 직접 하드코딩 되었고, 빠르고 효율적이지만 변경되지
    // 않는 것을 전재로 하는 특성이다.
    // String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어졌고, 힙에서 컴파일
    // 타임에는 알 수 없는 어느 정도 크기의 메모리 공간을 할당받아 내용물을 저장할 필요가 있다.
    // 1. 런타임에 운영체제로부터 메모리가 요청되어야 한다.
    // 이는 우리가 직접 수행한다. String::from이 필요한 만큼의 메모리를 호출하는 것.

    // 2. String의 사용이 끝났을 때 운영체제에게 메모리를 반납할 방법이 필요하다.
    // Rust는 }괄호가 닫힐 때 자동적으로 drop(자원을 해제함 free처럼)을 호출한다.
    
    let x = 5;
    let y = x;
    println!("x is: {x} and y is: {y}"); // x 값의 복사본이 y에 bound된다. 스택에 값이 푸쉬됨.

    let s1 = String::from("Hello");
    let s2 = s1;  // s1은 s2에 할당되면서 drop되고, double free를 예방한다.
    // println!("s1 is: {s1} and s2 is {s2}");  // 이 코드를 실행하면 value used here after move
    // 에러 발생한다. 이렇게 첫번째 변수를 두번째 변수에 할당하고 무효화 시키는 것을move라고 부름.
    // double free는 momory corruption (메모리 손상)의 원인이 되는데, 이는 보안 취약성 문제를
    // 일으킬 가능성이 있다.
    // 러스ㅡ트는 결코 자동적으로 데이터에 대한 "깊은" 복사본을 만들지 않아, 어떠한 자동적인
    // 복사라도 런타임 실행 과정에서 효율적이라 가정할 수 있다.
    
    // 그럼에도 힙 데이터를 깊이 복사하길 원한다면 clone이란 공용 메소드를 사용할 수 있다.
    
    let s1 = String::from("Hi there!");
    let s2 = s1.clone();
    println!("s1 is: {s1} and s2 is: {s2}");
    
    // 이렇게 힙 데이터를 깊이 복사할 수 있다.

    // String ptr, len, capacity(용량) 세 부분으로 이루어짐. 
    // ptr은 문자열의 내용물을 담고 있는 메모리의 포인터, 그 내용물의 길이와 용량
    // 이 데이터 그룹은 스택에 저장되고 내용물 담은 메모리는 힙 메모리에 있다.
    // len은 바이트 단위.
    
    let x = 5;
    let y = x;
    // 이 코드는 x 유효하며 y로 move 하지 않는 이유는:
    // 컴파일 타임에 결정되어 있는 크기의 타입은 스택에 모두 저장되기 때문에,
    // 실제 값의 복사본이 빠르게 만들어질 수 있다. 
    // 이는 y가 생성된 후에 x가 더 이상 유효하지 않도록 해야할 이유가 없다는 것.
    // 여기선 깊은 복사와 얕은 복사 간의 차이가 없다는 것으로, clone을 호출하는 것이 보통의 얕은
    // 복사와 아무런 차이점이 없어 이를 그냥 버릴 수 있다는 것이다.
    
    // 러스트는 스택에 저장할 수 있는 타입에 copy trait라 불리는 특별한 annotation을 가지고 있다.
    // 어떤 타입이 copy trait을 가지고 있다면 assign을 하면 복사되고 기존 값도 계속 쓸 수 있음.
    // Rust는 어떤 타입이나 그 타입이 가지고 있는 부분 중에서 Drop 트렛이 구현된 부분이 있다면 copy
    // 트레잇을 annotation 할 수 없게 한다.
    // 단순한 스칼라 값들의 묶음은  copy가 가능하고, allocation이 필요하거나 is some form of
    // resource면 copy를 사용할 수 없다.
    // 예시: u32 등 모든 정수형 타입들/ bool/ f64 등 float types/ copy가 가능한 타입만으로 구성된
    // 튜플들. (i32, i64)는 copy 되지만 (i32, String)은 copy 안 된다. 
    
    // 소유권과 함수:
    
    let s = String::from("ARRRR!!!");  // s가 스코프 안으로 들어왔다.
    takes_ownership(s);  // s값이 함수 안으로 이동했다
    // s는 더이상 유효하지 않다.
    let x = 5;  // x는 스코프 안으로 들어왔다.
    makes_copy(x);
    
    
    // 반환 값과 스코프:
    
    
    let s1 = gives_ownership();
    let s2 = String::from("Estoy muy cansada");
    let s3 = takes_and_gives_back(s2);  // s2는 takes~안으로 이동되었고,
    // 이 함수가 반환값을 s3으로도 이동시켰다.
    
   // 함수에게 값을 사용할 수 있도록 하되 소유권을 갖지 않도록 하고 싶다면? 함수의 본체로부터
   // 얻어진 결과와 더불어 우리가 넘겨주고자 하는 어떤 값을 다시 쓰고 싶어서 함께 반환받아야 한다면 
   // 꽤나 짜증날 것이다. 
   // 예제코드는 바보같아서 안 싣음. references라는 기능 이용하면 된다.
   

}  // scope가 끝나 drop되는 지점.
//여기서 s3은 스코프 밖으로 벗어났으며 drop이 호출됨, s2는 스코프 밖으로 벗어났지만 이동되었으므로
//아무 일도 일어나지 않는다. s1은 스코프 밖으로 벗어나서 drop 호출된다.

fn takes_ownership(some_string: String){ // some_string이 스코프 안으로 들어왔다.
    println!("{some_string}");
}  // 여기서 some_string이 스코프 밖으로 벗어났고 drop이 호출된다. 메모리는 해제된다.

fn makes_copy(i: i32){  // i가 스코프 안으로 들어왔다.
    println!("{i}");
}  // copy trait이 annotation된 scalar type이기 때문에 별다른 일 발생 X

fn gives_ownership() -> String{
    let some_string = String::from("Esta es regalo para ti.");  // some_string이 스코프 안으로 이동
    some_string  // some_string이 반환되고, 호출한 쪽의 함수로 이동된다.
}

fn takes_and_gives_back(a_string: String) -> String{  // a_string이 스코프 안으로 들어옴
    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동한다.
}
