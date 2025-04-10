#![allow(unused)]
pub struct Food {
    name: <name>,
    calories: [<value_in_kJ>, <value_in_kcal>],
    fats: <fats_in_g>,
    carbs: <carbs_in_g>,
    proteins: <proteins_in_g>,
    nbr_of_portions: <portions>
}
use json::JsonValue;

fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut total_calories = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        if let (Some(_kj), Some(kcal)) = (
            food.calories.get(0).and_then(|s| s.parse::<f64>().ok()),
            food.calories.get(1).and_then(|s| s.parse::<f64>().ok()),
        ) {
            total_calories += kcal * food.nbr_of_portions;
        }
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    fn round_to_two_decimals(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }

    let mut result = JsonValue::new_object();
    result["cals"] = round_to_two_decimals(total_calories).into();
    result["carbs"] = round_to_two_decimals(total_carbs).into();
    result["proteins"] = round_to_two_decimals(total_proteins).into();
    result["fats"] = round_to_two_decimals(total_fats).into();

    result
}
