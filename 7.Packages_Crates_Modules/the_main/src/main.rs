fn main() {
    println!(
        "I'm using the library: {:?}",
        the_lib::really_complicated_code(1, 2)
    );
    the_lib::sibling1::hello();
    the_lib::sibling2::hello();
}

// sibling modules일지라도 다른 파일에 구현되면 pub 없인 접근할 수 없음.
