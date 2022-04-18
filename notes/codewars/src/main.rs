// Area of a rect amirite
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("Hello, world!");

    let r = Rectangle { width: 10, height: 20 };
    println!("Area of r is: {}", r.area());
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
