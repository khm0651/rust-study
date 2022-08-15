#[derive(Debug)]
struct CubeSat {
    id: i64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

/// &self는 자신에 대한 읽기 전용 참조만 사용한다는 의미
/// CueeSat 인스턴스의 읽기, 쓰기 소유권 요청 대신 대여를 요청한다. (쓰기를 요청하는 이유는 벡터의 값을 수정하는 메서드이기 때문에)
/// msg는 Message인스턴스의 완전한 소유권을 요청한다.
impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("hello hamin"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    print!("msg: {:?}", msg);
}
