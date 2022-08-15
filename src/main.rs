#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: i64) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    /// 아래 로직이 성공 할 수 있는 이유는 원시타입은 Copy 트레이트를 구현하고 있기 때문에
    /// 복제하지 않고서는 사용할 수 없을 때에 한해 복제가 되기 떄문에 아래 로직이 실행이 되는 것이다.
    /// 원시타입은 '복사 의미'를 가지며 다른 타입은 '이동 의미'를 가진다.
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
