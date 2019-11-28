pub fn main() {
  let s = String::from("hello world!");
  let l = first_word(&s);
  println!("first word of '{}' ends at index {}", s, l);

  // cannot isert string literals as string
  // let l = first_word(&s);
  // println!("first word of '{}' ends at index {}", s, l);
  // however strings are string literals so otherway around it works
  let s = "hello world";
  let w = first_word_slice(s);
  println!("first word of '{}' is {}", s, w);
  let s = String::from("hello world!");
  let w = first_word_slice(&s);
  println!("first word of '{}' is {}", s, w);

  // slices can be used with other types too
  let a = [1, 2, 3, 4, 5];
  let s = &a[1..4];
  println!("s ({:?}) is a slice of a ({:?})", s, a);
}

// return the index of the end of the first word in a string
// this is not quite efficient, also if we wanted to make a function that returns a word from middle of a string it would need to return a tuple and various other problematiques.
fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes(); // turn string into bytes

  for (i, &item) in bytes.iter().enumerate() { // loop through each char in string as byte
    if item == b' ' { // look for a space
      return i; // return index if space found
    }
  }

  s.len() // if string is one word, return the length of string
}

// solution: getting position of a word with string slice!
// return the first word of a string
fn first_word_slice(s: &str) -> &str {
  let bytes = s.as_bytes(); // turn string into bytes

  for (i, &item) in bytes.iter().enumerate() { // loop through each char in string as byte
    if item == b' ' { // look for a space
      return &s[..i]; // return index if space found
    }
  }

  &s[..] // if string is one word, return the word
} 