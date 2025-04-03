pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum()  // Sum all the elements in the array/slice
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]  // Create an array of 32 elements, each set to 10
}

fn main() {
    // Create a vector with numbers 1 through 10
    let a = (1..=10).collect::<Vec<i32>>();  // Converts the range into a vector of i32
    // Create an array with 10 elements, all set to 5
    let b = [5; 10];  // Array of 10 elements, each set to 5

    // Print the sum of elements in vector `a`
    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    // Print the sum of elements in array `b`
    println!("The sum of the elements in {:?} is {}", b, sum(&b));
    // Print the array filled with 32 tens
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}
