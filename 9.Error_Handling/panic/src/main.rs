fn main() {
    // panic은 코드가 패닉을 일으킬 동작을 하거나,  panic! 매크로를 명시적으로
    // 호출하는 두 가지 경우에서 발생한다.
    // panic!("crash and burn");

    // panic! 백트레이스 이용하기:
    // 직접 매크로를 호출하는 대신 라이브러리로부터 panic! 호출이 발생할 때:
    
    let v = vec![1, 2, 3];
    v[99];
    // In C, attempting to read beyond the end of a data structure is undefined 
    // behavior. You might get whatever is at the location in memory that would
    // correspond to that element in the data structure, even though the memory 
    // doesn't belong to that structure.
    // This is called a "buffer overread" and can lead to security vulnerabilities
    // if an attacker is able to manipulate the index in such a way as to read data
    // they shouldn't be allowed to that is stored after the data structure.
    
    // Backtrace란, 어떤 지점에 도달하기까지 호출한 모든 함수의 목록을 말한다.
    // 백트레이스를 읽는 요령은 위에서부터 시작하여 여러분이 작성한 파일이 
    // 보일 때 까지 읽는 것이다.
    // 전후의 줄에는 핵심 러스트 코드, 표준 라이브러리, 이용중인 크레이트가 포함
    // 될 수 있다.

    
    // RUST_BACKTRACE=1 cargo run으로 backtrace 볼 수 있음.
    // In order to get backtraces with this information,
    // "Debug Symbols" must be enabled.
    // Debug Symbols are enabled by default when using cargo build or cargo run
    // without the --release flag, as we have here.
}
