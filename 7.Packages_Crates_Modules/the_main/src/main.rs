fn main() {
    println!(
        "I'm using the library: {:?}",
        the_lib::really_complicated_code(1, 2)
    );
    the_lib::sibling1::hello();
    the_lib::sibling2::hello();
    // the_lib을 디펜던시에 추가하지 않았다면 
    // pub mod the_lib 으로해당 모듈을 외부에서 접근 가능하게 선언해야됨.
}

// sibling modules일지라도 다른 파일에 독립적으로 구현되면 pub 없인 접근할 수 없음.
