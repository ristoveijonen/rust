// A program that calculates rectangles area in square pixels
// 1. we use separate height and width values to calculate area, this is bothersome since we need to keep two different variables in mind.
// 2. Tuples group the values, but obfuscate them. It's hard to remember if dimensions.0 is height or width.
// 3. with sturcts we are grouping the values, but also keeping the code readable.

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

// implement area calcultion as a method for Rectangle struct
impl Rectangle {
  fn area(&self) -> u32 { // first parameter of a method is always self, note &self: methods can take ownership, borrow, etc...
      self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool { // methods can take other parameters also
    self.width > other.width && self.height > other.height
  }
}

impl Rectangle { // the amount of impl blocks is not restricted
  // associated function
  fn square(size: u32) -> Rectangle { // associated functions do not take self as a parameter
    Rectangle { width: size, height: size }
  }
}

pub fn main() {
  let square = Rectangle::square(40);
  let width = 30;
  let height = 50;
  let tuple_rect = (width, height);
  let rect = Rectangle { width: 30, height: 50 };
  let rect_2 = Rectangle { width: 25, height: 15 };
  let rect_3 = Rectangle { width: 60, height: 45 };

  println!("The area of the rectangle is {} square pixels.", area_from_variables(width, height));
  println!("The area of the rectangle is {} square pixels.", area_from_tuple(tuple_rect));
  println!("The area of the rectangle is {} square pixels.", rect.area());
  println!("{:?}", rect);
  println!("Can rect 1 hold rect_2? {}", rect.can_hold(&rect_2));
  println!("Can rect 1 hold rect_3? {}", rect.can_hold(&rect_3));
  println!("square is {:#?}", square);
}

fn area_from_variables(width: u32, height: u32) -> u32 {
  width * height
}

fn area_from_tuple(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}
