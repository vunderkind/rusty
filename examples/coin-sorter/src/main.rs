enum Coin {
    Naira,
    Kobo,
}
fn main() {

    fn coin_match(coin:&Coin) -> u8 {
        match coin {
            Coin::Naira => 100,
            Coin::Kobo => 1,
        }
    }

    let coin = Coin::Naira;
    let kobo = Coin::Kobo;
    println!("The value of 1 naira is: {} kobo", coin_match(&coin));
    println!("The value of 100 kobo is: {} naira", coin_match(&kobo));

    plus_one(Some(4));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}