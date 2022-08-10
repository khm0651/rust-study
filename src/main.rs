fn main() {
    /**
        슬라이스란 소유권을 갖지 않는 또다른 데이터 타입
        슬라이스는 컬렉션(collection) 전체가 아닌
        컬렉션의 연속된 일련의 요소들을 참조할 수 있게 합니다.
        최종 시나리오. 스트링을 입력 받아 그 스트링에서 찾은 첫 번째 단어를 반환하는 함수 작성.
        시나리오1. 슬라이스를 쓰지 않고 첫 번째 단어의 인덱스를 반환한 뒤 시간이 지남에 따라 유효 하지 않은 값으로 변경 후 기댓값과 결과값 비교
        시나리오2. 슬라이스를 쓰고 첫 번째 단어의 인덱스를 반환한 뒤 시간이 지남에 따라 유효 하지 않은 값으로 변경 후 기댓값과 결과값 비교
    **/
    scenario_one();
    slice_exercise();
    scenario_two();
}

fn scenario_two() {
    let mut s = String::from("hamin having nice time with safe happy coding");

    let word = first_word_with_slice(&s);

    // ...
    // ...
    // ...
    // ...
    // ...
    // ...
    // ...

    //시간이 지남에 따라 s에 있는 값 변경
    s = String::from("haha now is future");

    /**
        컴파일시 에러 발생.
        빌림 규칙에서 만일 무언가에 대한 불변 참조자를 만들었을 경우
        가변 참조자를 만들 수 없다.
         s = String::from("haha now is future"); 이 코드는
        가변 참조자를 갖기 위한 시도를 하고 빌림 규칙에 의해 실패함.
        이러한 종류의 오류 전체를 컴파일 타임에 발견하기 때문에 안전하게 코딩을 할 수 있다.
    **/
    let expect = String::from("hamin");
    assert_eq!(word, expect);
}

/**
    예상.. 수명 관련 문제로 안되는 듯 하다.. 악!
**/
// fn first_word_with_slice2(s: String) -> &'static str {
//     let bytes = s.as_bytes();
//     let copy = s.clone();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &copy[0..i];
//         }
//     }
//
//     &copy[..]
// }

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice_exercise() {
    let s = String::from("hamin having nice time with safe happy coding");

    /**
       아래는 스트링 슬라이스를 의미함. 스트링 슬라이스란 String의 일부분에 대한 참조자를 의미
       이는 전체 String의 참조자를 갖는 것과 비슷 하지만 start..end 문법으로 end를 포함하지 않는 연속된 범위를 기술하여
       String의 일부분에 대한 참조만 한다.
    **/
    let hamin = &s[0..5];
    let having = &s[6..12];

    assert_eq!(hamin, "hamin");
    assert_eq!(having, "having");

    /**
        러스트의 .. 범위 문법을 사용하여 0번째 인덱스에서 부터 사용하는 두가지 케이스
        (러스트에서 동일한 변수명을 허용한다.)
    **/
    let hamin_having = &s[0..12];
    assert_eq!(hamin_having, "hamin having");
    let hamin_having = &s[..12];
    assert_eq!(hamin_having, "hamin having");

    /**
        String의 전체 스트링의 슬라이스를 만드는 방법 세가지
    **/
    let len = s.len();
    let hamin_having_nice_time_with_safe_happy_coding = &s[0..len];
    assert_eq!(
        hamin_having_nice_time_with_safe_happy_coding,
        "hamin having nice time with safe happy coding"
    );
    let hamin_having_nice_time_with_safe_happy_coding = &s[..];
    assert_eq!(
        hamin_having_nice_time_with_safe_happy_coding,
        "hamin having nice time with safe happy coding"
    );
    let hamin_having_nice_time_with_safe_happy_coding = &s[0..];
    assert_eq!(
        hamin_having_nice_time_with_safe_happy_coding,
        "hamin having nice time with safe happy coding"
    );
}

fn scenario_one() {
    let mut s = String::from("hamin is fool guy");

    let word_index = first_word_without_slice(&s);

    // ...
    // ...
    // ...
    // ...
    // ...
    // ...
    // ...

    //시간이 지남에 따라 s에 있는 값이 바뀌게 되고 버그 발생
    s = String::from("haha now is future");

    let mut result = String::from("");

    /**
        첫번째 시도
        ^^^^ `String` cannot be indexed by `usize` 오류 발생 String 은 Index<usize>를 구현 하지 않고 있음
    **/
    // for i in 0..word_index {
    //     result.push_str(&s[i]);
    // }
    for i in 0..word_index {
        result.push_str(&(s.as_bytes()[i] as char).to_string());
    }

    /**
        심각한 오류다. 개발자는 hamin을 기대하고 쭉 코드 작성을 한 뒤 결과를 보면 틀린 값을 가지고있다.
        이는 런타임시에 발견할 확률이 매우 크기 때문에 안전 하지 못하다.
    **/
    let expect = String::from("hamin");
    assert_eq!(result, expect);
}

/**
    공백문자를 찾지 못한다면 한 단어라는 의미이기 때문에 전체 스트링이 반환
**/
fn first_word_without_slice(s: &String) -> usize {
    let bytes = s.as_bytes(); //바이트 배열로 변환

    //enumerate는 튜플을 반환
    //.iter().enumerate()의 요소에 대한 참조자를 갖는 것이므로 &을 사용
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i - 1;
        }
    }

    s.len()
}
