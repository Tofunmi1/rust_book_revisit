#[derive(Debug)]
enum UsState {
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}
fn main() {
    let coin: Coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);
}
