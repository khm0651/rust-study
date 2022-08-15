#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // '대기 중' ...

    ///     아래와 같은 오류가 발생.
    ///     CubeSat type을 가지고있는 sat_a 변수는 check_status함수에 소유권이 이동이 되었다.
    ///     이는 sat_a 객체의 소유권이 check_status()로 이동하지만 main()으로 돌아오지 않는다는 뜻을 의미.
    ///     따라서 sat_a는 더 이상 해당 객체의 소유자가 아니어서 해당 접근은 무효가 된다.
    // 16 |     let sat_a = CubeSat { id: 0 };
    //    |         ----- move occurs because `sat_a` has type `CubeSat`, which does not implement the `Copy` trait
    // ...
    // 20 |     let a_status = check_status(sat_a);
    //    |                                 ----- value moved here
    // ...
    // 27 |     let a_status = check_status(sat_a);
    //    |                                 ^^^^^ value used here after move
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
