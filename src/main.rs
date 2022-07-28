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

    println!(
        "a bagel with {} calories",
        calc_calorie_from_elements(1.0, 1.0, 1.0)
    )
}

fn calc_calorie_from_elements(flour_g: f32, sugar_g: f32, butter_g: f32) -> f32 {
    let flour_cal = flour_g * 3.55;
    let sugar_cal = sugar_g * 3.86;
    let butter_cal = butter_g * 7.16;
    return flour_cal + sugar_cal + butter_cal;
}
