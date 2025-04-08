use changes::*;

fn main() {
    // Initialize an array of Light objects
    let mut lights = ["living_room", "bedroom", "rest_room"]
        .iter()
        .map(|&alias| Light::new(alias))
        .collect::<Vec<Light>>();

    // Display the initial brightness of the first light
    println!("brightness = {}", lights[0].brightness);

    // Change the brightness of the 'living_room' light to 200
    change_brightness(&mut lights, "living_room", 200);

    // Display the new brightness of the first light
    println!("new brightness = {}", lights[0].brightness);
}
