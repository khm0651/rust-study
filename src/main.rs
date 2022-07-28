fn main() {
    let bagel = 10;
    let ready = 2;
    println!("ready to serve {} of {} bagels", ready, bagel);

    println!("serving..!!");
    let need = bagel - ready;
    println!("now {} bagels left to serve", need);
}
