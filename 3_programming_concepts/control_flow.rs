fn main() {
  if_expressions();
  loops();
}

fn if_expressions() {
  let number = 6;

  // If condition must always evaluate to bool.
  // Non-bool types are not converted to bool. 
  // ie. 1 !== true and 0 !== false in rust.
  // if is a statement and can be used right side of an expression.
  // When if returns a value, the values MUST be of the same type.
  let result = if number < 5 {
    "less than 5"
  } else if number == 5 {
    "exactly 5"
  } else {
    "higher than 5"
  };

  println!("number is {}", result);
}

fn loops() {
  let mut counter = 0;
  // Rust has 3 different types of loops: loop, while and for

  // LOOP
  // Will loop until explicitly told to stop.
  // Check out an exmple in guessing_game.
  // loop is a statement and can be used right side of an expression.
  let result = loop {
    counter += 1;
    println!("Loop will be broken when counter is 3! counter is {}", counter);
    if counter == 3 {
      break counter * 2;
    }
  };
  println!("result is {}", result);

  // WHILE
  // while is a combination of loop and if statement
  // while will stop looping when the set condition is true
  while counter != 0 {
    counter -= 1;
    println!("count is now {}", counter);
  }

  // FOR
  let a = [0, 1, 2, 3, 4, 5];
  for element in a.iter() {
    println!("element is {}", element);
  }
  // The while counter counting down could also be done in a for loop
  for number in (1..4).rev() {
    println!("count is now {}", number);
  }
}