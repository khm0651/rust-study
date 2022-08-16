use std::fmt::format;

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn recv(&self, mailBox: &mut Mailbox) -> Option<Message> {
        mailBox.deliver(&self)
    }
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg)
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

struct GroundStation {}

/// &self는 자신에 대한 읽기 전용 참조만 사용한다는 의미
/// CueeSat 인스턴스의 읽기, 쓰기 소유권 요청 대신 대여를 요청한다. (쓰기를 요청하는 이유는 벡터의 값을 수정하는 메서드이기 때문에)
/// msg는 Message인스턴스의 완전한 소유권을 요청한다.
impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailBox: &mut Mailbox, msg: Message) {
        mailBox.post(msg)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };

    let base = GroundStation {};

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message {
            to: sat.id,
            content: format!("i'm hamin {}", sat.id),
        };
        base.send(&mut mail, msg)
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}
