use rand::rngs::mock::StepRng;
use rand::RngCore;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;

fn parse_log(line: &'static str) -> (Event, Message) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() == 1 {
        return (Event::Unknown, String::from(line));
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, String::from(rest)),
        "DELETE" | "delete" => (Event::Delete, String::from(rest)),
        _ => (Event::Unknown, String::from(line)),
    }
}

fn main() {
    // split_n_test();
    // split_test();
    // enum_test();
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/221111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("${:?}", parse_result);
    }
}

fn split_n_test() {
    let s = String::from("hamin is coding hard");
    let parts: Vec<_> = s.splitn(2, ' ').collect(); // (_) 는 러스트에게 자동으로 추론하라고 하는 것.
    assert_eq!(parts, ["hamin", "is coding hard"]);
}

fn split_test() {
    let s = String::from("hamin is coding hard");
    let parts: Vec<_> = s.split(' ').collect();
    assert_eq!(parts, ["hamin", "is", "coding", "hard"]);
}

#[derive(Debug)]
enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
enum Card {
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Ace(Suit),
    Pip(Suit, usize),
}

fn enum_test() {
    let mut card_list = vec![
        Card::Ace(Suit::Clubs),
        Card::Queen(Suit::Spades),
        Card::Pip(Suit::Clubs, 8),
        Card::Jack(Suit::Clubs),
        Card::King(Suit::Hearts),
        Card::Pip(Suit::Clubs, 1),
        Card::Pip(Suit::Clubs, 2),
        Card::Pip(Suit::Clubs, 3),
        Card::Pip(Suit::Clubs, 7),
    ];

    for i in 0..9 {
        let card = &card_list[i];
        match card {
            Card::King(_) => {
                println!("i'm king {:?}", card)
            }
            Card::Queen(_) => {
                println!("i'm Queen {:?}", card)
            }
            Card::Jack(_) => {
                println!("i'm Jack {:?}", card)
            }
            Card::Ace(_) => {
                println!("i'm Ace {:?}", card)
            }
            Card::Pip(_, _) => {
                println!("i'm Pip {:?}", card)
            }
        }
    }
}
