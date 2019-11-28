
mod what_is_ownership;
mod references_and_borrowing;
mod the_slice_type;

fn main() {
  what_is_ownership::scope();
  print!("\n\n");
  references_and_borrowing::main();
  print!("\n\n");
  the_slice_type::main();
}
