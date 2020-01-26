// This is the crate library root.
// Rust knows to run lib.rs files from the root of a crate.

mod back_of_house {
  mod child_mod {
    fn child_mod_fn() {
      // Calling super from a child module would not work for using serve_order(), for it is not referencing to back_of_house, but child_mod.
      // super::serve_order();

      // However cook_order() can be referenced similarly as serve_order in fix_incorrect_order.
      super::cook_order();
    }
  }

  fn fix_incorrect_order() {
    // Cook order is inside the parent of this fn, so it is visible.
    cook_order();

    // super:: can be used to reference one step outsie of the mod root.
    // In a directory call it's equivalent to ../serve_order
    super::serve_order();
  }

  fn cook_order() {}
}

fn serve_order() {}

// Take into scope front_of_house.rs
mod front_of_house;

// pull public reference of hosting from front_of_house.rs
// hosting is already taken to use so we need to rename it if it is to be used.
pub use crate::front_of_house::hosting as ExteranalHosting;

// Public function is visible to all other crates, modules etc. outside of this file
pub fn eat_at_restaurant() {
  // Absolute path
  // Strats from the crate root
  crate::front_of_house::hosting::add_to_waitlist();
  // Relative path
  // Starts from this file
  front_of_house::hosting::add_to_waitlist();
}

// Public structs and enums
mod back_of_house_2 {
  pub struct Breakfast {
    // toast is public
    pub toast: String,
    // seasonal_fruit is private, even though the struct is public.
    seasonal_fruit: String,
  }

  // public enums expose all their contents ar public if the enum is public.
  pub enum Appetizer {
    Soup,
    Salad,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
}

pub fn eat_breakfast() {
  // Order a breakfast in the summer with Rye toast
  let mut meal = back_of_house_2::Breakfast::summer("Rye");
  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");

  // The enum Appetizer is public, thus Soup and Salad are both public and the following is possible.
  let order1 = back_of_house_2::Appetizer::Soup;
  let order2 = back_of_house_2::Appetizer::Salad;
}

// Bringing a module into scope
mod front_of_house_3 {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

// Bringing hosting from front_of_house_3 to scope
use crate::front_of_house_3::hosting;
// Bringing HashMap to scope
use std::collections::HashMap;
// Bringing fmt and io to scope
use std::fmt;
use std::io;

pub fn eat_at_restaurant_2() {
  // instead of prefixing the whole path crate::front_of_house_3::hosting::add_to_waitlist() for all of the functions below we access them by bringing it to scope on line 105
  // By bringing hosting to scope, rather than add_to_waitlist, it is clear that fn add_to_waitlist() is outside of the current scope for the reference is allways visible, while still being as little repetition as possilbe.
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();

  // It's customary for structs, enums and other items however to be brought to scope with full path.
  // See line 107
  let mut map = HashMap::new();
  map.insert(1, 2);

  // There is a exception when bringing items with same name to scope.
  // The below would not work if Results were brought to scope like
  // use std::fmt::Result;
  // use std::io::Result;
  fn function1() -> fmt::Result {}
  fn function2() -> io::Result<()> {}
  // The above problem could be averted also by renaming other one of the Results
  // use std::fmt::Result;
  // use std::io::Result as IoResult;

  // Using pub use in an export makes the export usable to all external crates as well.
  
  // use syntax
  // Imports from the same crate can be combined
  // use std::io;
  // use std::cmp::Ordering;
  // use std::{cmp::Ordering, io}
  
  // self reference in use
  // use std::io;
  // use std::io::Write;
  // use std::io::{self, Write}
  
  // Global operator, imports everything from a crate
  // use std::collections::*;
}
