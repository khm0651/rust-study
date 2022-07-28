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
    );

    let bagel1: [f32; 3] = [5.0, 10.0, 1.0];
    let bagel2: [f32; 3] = [2.0, 27.0, 40.0];
    let bagel3: [f32; 3] = [3.0, 1.0, 5.0];

    let bagel_box = [bagel1, bagel2, bagel3];

    for bagel in bagel_box {
        println!(
            "a bagel with {} calories",
            calc_calorie_from_elements(bagel[0], bagel[1], bagel[2])
        );
    }

    let mut bagel_calories: Vec<f32> = bagel_box
        .iter()
        .map(|bagle| calc_calorie_from_elements_array(*bagle))
        .collect();

    for calorie in bagel_calories {
        println!("a bagel with {} calories", calorie);
    }
}

fn calc_calorie_from_elements_array(flour_sugar_butter: [f32; 3]) -> f32 {
    calc_calorie_from_elements(
        flour_sugar_butter[0],
        flour_sugar_butter[1],
        flour_sugar_butter[2],
    )
}

fn calc_calorie_from_elements(flour_g: f32, sugar_g: f32, butter_g: f32) -> f32 {
    let flour_calories = flour_g * 3.55;
    let sugar_calories = sugar_g * 3.86;
    let butter_calories = butter_g * 7.16;
    ((flour_calories + sugar_calories + butter_calories * 100.0) as i32) as f32 / 100.0
}
