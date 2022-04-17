fn main() {
    println!("Hello, world!");
}

// a function that takes a string and returns a greeting
// has to return it in title case
fn hello(name: &str) -> String {
    if name.len()>0 {
        let new_name = String::from(name).to_uppercase();
        let mut ix = new_name.clone().to_lowercase();
        let something = &new_name[..1];
        ix.replace_range(..1, something);
        format!("Hello, {}!",ix)
    }
    else {
        return String::from("Hello, World!")
        }
}

// Given a non-empty array of integers, return the result of multiplying the values together in order.
fn grow(nums: Vec<i32>) -> i32 {
    let mut tally = 1;
    for (_,j) in nums.iter().enumerate() {
        tally = tally * j
    }
    return tally
}