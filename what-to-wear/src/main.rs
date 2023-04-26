use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let shoes = [
        "sneakers",
        "boots",
        "sandals",
        "loafers",
    ];

    let tops = [
        "t-shirt",
        "sweater",
        "blouse",
        "button-down shirt",
    ];

    let bottoms = [
        "jeans",
        "shorts",
        "skirt",
        "trousers",
    ];

    let mut rng = thread_rng();

    let selected_shoe = shoes.choose(&mut rng).unwrap();
    let selected_top = tops.choose(&mut rng).unwrap();
    let selected_bottom = bottoms.choose(&mut rng).unwrap();

    println!(
        "Today's outfit: {} with a {} and {}.",
        selected_shoe, selected_top, selected_bottom
    );
    // format!(
    //     "Today's outfit: {selected_shoe} with a {selected_top} and {selected_bottom}." 
    // );
}
