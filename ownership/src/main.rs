fn main() {
  scope();
}

// In rust an item is only valid inside the scope it's declared in.
fn scope() { // scopre of scope().
  // s goes to the stack here, because it's size is known at compile time.
  let s = "hello"; // s is in scope as long as it stays inside of scope().
  println!("{} from stack", s);
  
  // this decöartation of s will be added to the heap.
  let mut s = String::from(s);
  println!("{} from heap", s);
  s.push_str(", World!");
  println!("{}", s);
} // end of scope of scope().