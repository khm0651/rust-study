use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct GroundStation {
    radio_req: f64,
}

/// RC<T>는 스레드에 대해 안전하지 않다.
/// 다중 스레드 코드에서는 RC<T>를 Arc<T>로, Rc<RefCell<T>>를 Arc<Mutex<T>>로 대체하는 것이 낫다.
/// arc는 원자적 참조 카운터(atomic reference counter)를 의미

fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(GroundStation { radio_req: 10.0 }));

    println!("base: {:?}", base);

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_req -= 5.0;
        println!("base2: {:?}", base_2);
        println!("base in inner fn: {:?}", base); // inner fn 내에서 base_2에 대여 되었으며 접근 가능하지 않음

        //스코프가 종료되고 base_2변수에 대여한 소유권을 다시 쓸 수 있게 됌
    }

    println!("base in main fn: {:?}", base); //소유권을 다시 얻어서 접근 가능함.

    let mut base_3 = base.borrow_mut();
    base_3.radio_req += 7.0;

    println!("base3: {:?}", base_3);
    println!("base in inner fn: {:?}", base); // main fn 내에서 base_3에 대여 되었으며 접근 가능하지 않음
}
