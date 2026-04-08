
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,

}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America."))
        }
        else{
            Some(format!("{state:?} is relatively new."))
        }
    }
    else{
        None
    }
}

fn main() {

    let mut count = 0;

    let coin_one = Coin::Penny;
    let coin_two = Coin::Quarter(UsState::Alaska);
    let coin_three = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = coin_two {
        println!("Coin is from: {state:?}")
    }
    else{
        count += 1;
    }


    let described_coin = describe_state_quarter(coin_three);
    //println!("{described_coin}");
}
