fn main() {
  // main() is always run first in a .rs file.
  // functions use snakecase naming convention.
  println!("snakecase_means_small_letters_connected_with_underscores");
  let result = second_function(5, 10);
  println!("result is {}", result);
}

// Function parameters MUST be typed
// Functinos can alos return values. The return MUST be typed
fn second_function(x: i32, y: i32) -> i32 {
  println!("x is {}", x);
  println!("y is {}", y);

  // expressions do not have ; at the end and return a value.
  let z = {
    let x = 4; // statements have ; at the end and do not return values.
    x + 1
  };
  println!("z is {}", z);

  z
}