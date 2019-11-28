fn main() {
  // Rust has 4 scalar values: integers, floating-point numbers, Booleans, and characters
  
  // INTEGERS
  // Are either signed (i) or unsigned (u).
  // Unsigned integers can only store positive values.
  // Signed integers have a sign before the value: either + or -.
  // Integers can be stored as 8, 16, 32, 64, 128 bit values or as "arch".
  // Arch detremines the integers bit size, either 32 or 64, based on the architecture of the running computer.
  // Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses.
  // Unsigned variants can store numbers from 0 to 2n - 1, where n is the number of bits that variant uses.
  // 32-bit is genrally the fastest, even on 64-bit machines.
  // Underscore can be used as a visual separator 1000 = 1_000
  // If integer is assigned a value outside of the maximum or minimum of the variants bytesize the integer will be assigned lowest value it can hold in a compiled program. ie. let integer: u8 = 256 --> actually 0
  // Rusts default integer type is i32.
  let _default = 34;

  // FLOATING-POINT
  // Rust has two floating point types: f32 and f64
  // Rust defaults to f64 since it's roughly the same speed as f32 on modern computers, but is more precise.
  let _x = 3.2; // f64
  let _y: f32 = 1.0; // f32 

  // BOOLEAN
  // Either true or false.
  // Takes up one byte.
  // Can be annotated as bool.
  let _true = true;
  let _false: bool = false;

  // CHARACTER
  // Takes up 4 byes.
  // Specified with single quotes to separate it from strings.
  // Represents a Unicode Scalar Value --> Can represent special chars from any language or even Japanese, Korean etc. Can even represent emojis.
  let c = 'z';
  println!("c is {}", c);
  let z = 'ℤ';
  println!("z is {}", z);
  let heart_eyed_cat = '😻';
  println!("heart_eyed_cat is {}", heart_eyed_cat);

  // NUMERIC OPERATIONS
  // addition
  let sum = 5 + 2;
  println!("sum is {}", sum);
  // substraction
  let difference = 5 - 2;
  println!("difference is {}", difference);
  // multiplication
  let product = 5 * 2;
  println!("product is {}", product);
  // division
  let quotient = 5.2 / 2.5;
  println!("quotient is {}", quotient);
  // remainder
  let remainder = 5 % 2;
  println!("remainder is {}", remainder);

  // Rust has two main compound types: tuples and arrays

  // TUPLE
  // Types can differ.
  // Fixed length.
  // tuples can be destruturized
  let tup: (i32, f64, u8) = (500, 6.4, 1); // Type annotations are optional.
  println!("tup is {:?}", tup);
  let (_x, _y, z) = tup;
  println!("z is {}", z);
  // bindig can be accessed by using a period and index of the value.
  println!("tup index 0 is {}", tup.0);
  println!("tup index 1 is {}", tup.1);
  println!("tup index 2 is {}", tup.2);

  // ARRAY
  // Every element MUST have same type
  // Fixed length.
  // Typed by adding type and the length of th array inside [].
  // Trying to access an Array's index outside of it's length will compile, but produce a runtime error.
  let _a: [i32; 5] = [1, 2, 3, 4, 5];
  // If you want to create an Array with the same value you can do it by providing the value and the lenght separated by ;.
  let a = [3; 5];
  // Values can be accessed by index
  let _third = a[2];
}
