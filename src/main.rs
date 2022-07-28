const STARTING_BAGELS: i32 = 10;
const READY_TO_SERVE_BAGELS: i32 = 2;

fn main() {
    let mut bagel = STARTING_BAGELS;
    println!(
        "ready to serve {} of {} bagels",
        READY_TO_SERVE_BAGELS, bagel
    );

    println!("serving..!!");
    bagel -= READY_TO_SERVE_BAGELS;
    println!("now {} bagels left to serve", bagel);
}
