mod front_of_house {
    pub mod hosting {  // 모듈 내에는 다른 모듈을 넣을 수 있다.
    // 러스트에서는 모든 아이템이 기본적으로 부모 모듈에 대해 비공개이다.

    // 자식 모듈 내 아이템은 부모 모듈 내 아이템을 사용할 수 있다. 
    // 자식 모듈의 세부 구현은 감싸져서 숨겨져 있지만, 자식 모듈 내에서는 
    // 자신이 정의된 컨텍스트를 볼 수 있기 때문이다.
        pub fn add_to_waitlist() {}  
        // 모듈의 pub 키워드는 상위 모듈이 해당 모듈을 가리킬 수 있도록 할 뿐,
        // 그 내부 코드에 접근하도록 하는 것은 아니다.
        // 모듈은 단순한 컨테이너이기 때문에 모듈을 공개하는 것 만으로 할 수 있는 것은 
        // 별로 없으며, 여기에 더해서 모듈이 가지고 있는 아이템도 마찬가지로 공개해야한다.
        // 모듈에는 구조체, 열거형, 상수, 트레이트, 함수 등의 아이템 정의또한 가질 수 있다.

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
        // 위 세 모듈은 sibling 관계로, 동일한 모듈 내에 정의되어 있음을 말한다.
        // 또한 위 세 모듈은 serving 모듈의 자식이며, seving은 세 모듈의 부모이다.
        // 전체 모듈 트리 최상위에 crate라는 모듈이 암묵적으로 위치한다.
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로: 일반적으로 선호하는 경로.
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
    // front_of_house 모듈은 크레이트 루트 내에 정의되어 있다. 
    // 이는 공개가 아니지만, eat_at_restaurant() 함수와 front_of_house 모듈은 같은 모듈 내에
    // 정의되어 있으므로 (sibling 관계이므로) 서로 참조할 수 있다.
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
                // Breakfast 구조체는 비공개 필드를 갖고 있기 때문에,
                // Breakfast 인스턴스를 생성할 공개 연관 함수 (여기선 summer)을 반드시
                // 제공해야 한다. 없을 경우, eat_at_restaurant 함수에서
                // Breakfast 인스턴스를 생성할 수 없다.
                // 비공개 필드인 seasonal_fruit 필드의 값을 지정할 방법이 없기 때문에.
            }
        }
    }

    pub fn eat_at_restaurant() {
        // 호밀(Rye) 토스트를 곁들인 여름철 조식 주문하기
        let mut meal = back_of_house::Beakfast::summer("Rye");
        // 먹고 싶은 빵 바꾸기
        meal.toast = String::from("Wheat");
        println!("I'd like P{ toast please", meal.toast);

        // 식사와 함께 제공되는 계절 과일은 조회나 수정이 허용되지 않는다.
        // 아래의 주석을 해제하면 컴파일되지 않는다.
        // meal.seasonal_fruit = String::from("Blueberries");

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
        // Appetizer 열거형을 공개하였으니, eat_at_restaurant 함수에서
        // Soup, Salad variants를 사용할 수 있다.
        // 열거형은 그 배리언트가 공개되지 않는다면 큰 쓸모가 없다.
        // 구조체는 필드를 공개로 하지 않는 것이 종종 유용하므로, pub을 명시하지 않는 한
        // 기본적으로 모든 것이 비공개라는 일반적인 규칙을 따른다.
    }

    pub enum Appetizer {
        Soup,
        Salad,
    } 
    // 반대로, enum은 공개로 지정할 경우 모든 variants가 공개된다.

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
        // super로 시작하면 현재 모듈 혹은 크레이트 루트 대신 자기 부모 모듈로부터
        // 시작되는 상대 경로를 만들 수 있다.
        // 여기서 super은 back_of_house의 부모 모듈, 즉 루트를 의미한다.
        // back_of_house 모듈과 deliver_order 함수는 크레이트 모듈 구조 변경 시 서로의
        // 관계를 유지한 채 함께 이동될 가능성이 높다.
        // 그러므로 super을 사용하면, 차후에 다른 모듈에 이동시키더라도
        // 수정해야 할 코드를 줄일 수 있다.
    }

    fn cook_order() {}
}

// 모듈 트리는 src/lib.rs 내에 정의되어야 한다.
// 그러면 바이너리 크레이트 내에서는 패키지 이름으로 시작하는 경로를 사용함으로써
// 모든 공개 아이템을 사용할 수 있다.
// 바이너리 크레이트는 완전히 외부에 있는 다른 크레이트가 이 라이브러리 크레이트를
// 사용하는 식과 동일하게 이 라이브러리 크레이트의 사용자가 된다.
// 즉 공개 API만 사용할 수 있다! 
