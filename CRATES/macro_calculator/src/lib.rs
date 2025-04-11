use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        // Extract the kcal string and remove "kcal"
        let kcal_str = food.calories[1].replace("kcal", "");
        let kcal: f64 = kcal_str.parse().expect("Invalid calorie format");

        total_cals += kcal * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    // Round helper
    fn round(value: f64) -> f64 {
        let rounded = (value * 100.0).round() / 100.0;
        if (rounded * 10.0) % 1.0 == 0.0 {
            (rounded * 10.0).round() / 10.0
        } else {
            rounded
        }
    }

    json::object! {
        "cals" => round(total_cals),
        "carbs" => round(total_carbs),
        "proteins" => round(total_proteins),
        "fats" => round(total_fats)
    }
}
