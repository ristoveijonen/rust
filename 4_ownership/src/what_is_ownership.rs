// In rust an item is only valid inside the scope it's declared in.
pub fn scope() { // scopre of scope().
  // s goes to the stack here, because it's size is known at compile time.
  let s = "hello"; // s is in scope as long as it stays inside of scope().
  println!("{} from stack", s);
  
  // this decöartation of s will be added to the heap.
  let mut s = String::from(s);
  println!("{} from heap", s);
  s.push_str(", World!");
  println!("{}", s);

  // Copying integers creates two values, since they are simple values with known sizes
  // Below integers are both allocated to the stack and will be accessible until end of scope.
  let x = 5;
  let y = x;
  println!("x is {}. y is also {}", x, y); // both x and y will be accessible here
  // Copying strings is different. String values are saved as pointers to the heap.
  // Memory from heap can only be freed once, so the last declared value will be the one to hold the value until end of scope.
  let s1 = String::from("hello");
  let s2 = s1;
  println!("s2 is {}", s2);
  // uncommenting the below println! will throw an error!
  // println!("s1 is {}", s1);

  // To make s1 and s2 accessible until end of scope s2 needs to be a clone of the s1 variable.
  let s1 = String::from("hello");
  let s2 = s1.clone();
  println!("s1 is {}. s2 is also {}!", s1, s2);
  
  // These types are simple and stored in the stack = default is to copy the value.
  // all integer types
  // bool
  // floating points
  // char
  // tuples that only have types of the above

  let s = String::from("hello"); // s comes into scope
  takes_ownership(s); // s is moved out of scope

  let i = 5; // i comes into scope
  make_copy(i); // i is copied and does not lose it's scope
  println!("i is still in scope {}", i);

  let s1 = gives_ownership(); // s comes into scope

  let s2 = takes_ownership_and_gives_ownership(s1); // s1 is moved out of scope and s2 comes into scope

  // example of returning multiple values
  let (s2, len) = calculate_length(s2);
  println!("s2 is {} and its len is {}", s2, len);
} // end of scope of scope().

fn takes_ownership(string: String) {
  println!("{}", string);
}

fn make_copy(int: i32) {
  println!("{}", int);
}

fn gives_ownership() -> String {
  let s = String::from("hello again");
  s
}

fn takes_ownership_and_gives_ownership(string: String) -> String {
  string
}

fn calculate_length(string: String) -> (String, usize) {
  let len = string.len();
  (string, len)
}
