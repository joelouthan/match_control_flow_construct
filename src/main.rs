#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama
    Alaska
    Arizona
    Arkansas
    California
    Colorado
    Connecticut
    Delaware
    Florida
    Georgia
    Hawaii
    Idaho
    Illinois
    Indiana
    Iowa
    Kansas
    Kentucky
    Louisiana
    Maine
    Maryland
    Massachusetts
    Michigan
    Minnesota
    Mississippi
    Missouri
    Montana
    Nebraska
    Nevada
    New Hampshire
    New Jersey
    New Mexico
    New York
    North Carolina
    North Dakota
    Ohio
    Oklahoma
    Oregon
    Pennsylvania
    Rhode Island
    South Carolina
    South Dakota
    Tennessee
    Texas
    Utah
    Vermont
    Virginia
    Washington
    West Virginia
    Wisconsin
    Wyoming
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {}