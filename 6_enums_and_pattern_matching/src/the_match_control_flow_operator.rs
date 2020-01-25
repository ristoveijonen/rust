enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => {
        println!("Found a penny!");
        1
      },
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(state) => {
        println!("A US state quarter from {:?}!", state);
        25
      },
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    Some(i) => Some(i + 1),
    None => None
  }
}

pub fn main() {
  let penny = Coin::Penny;
  value_in_cents(penny);

  let alabama_quarter = Coin::Quarter(UsState::Alabama);
  value_in_cents(alabama_quarter);

  let some_number: Option<i32> = Some(13);
  println!("Adding one to some_number is: {:?}", plus_one(some_number));
  let none: Option<i32> = None;
  println!("Adding one to none is: {:?}", plus_one(none));
}