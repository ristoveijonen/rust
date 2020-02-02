pub fn strings() {
  let _s = String::new();
  let data = "initial data";

  // These all do the exact same thing
  let _s = data.to_string();
  let _s = "initial data".to_string();
  let _s = String::from("initial data");

  // Strings are UTF-8 encodid, so all manner of characters are possible to use.
  let s = String::from("こんにちは");
  println!("{}", s);

  // push_str doesn't take ownership
  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  // s2 is accessible here
  println!("s1 is {}, s2 is {}", s1, s2);

  // adding two strings with '+', takes ownership
  let s1 = String::from("foo");
  let s2 = String::from("bar");
  // rust cannot add two String values together (s1 + s2), thus s1 + &s2.
  // add takes ownership of s1 and adds &s2 to it. 
  let s3 = s1 + &s2; // s1 and is no longer after this line, s2 is only a reference so it lives on.
  println!("s2 is {} and s3 is {}", s2, s3);

  // More cmoplicated string concatenations can get a bit hard to read when using +
  let s1 = String::from("tic");
  let s2= String::from("tac");
  let s3 = String::from("toe");
  let s = s1 + "-" + &s2 + "-" + &s3;
  println!("{}", s);

  // format! can also be used. it is much more readable and doesn't take ownership of the variables
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("{}", s);

  // Unlinke many other programming languages, rust does not support accessing string characters by integer index.
  // For exmaple in JS one would expect s[0] === t, in rust one would get an error trying that
  // This is because rust encodes strings as UTF-8 and saves the length of a string as the length of bytes it takes.
  // So instead of returning the byte value of a character when requesting an index of a string and causing possible bugs and errors for unexpected values, rust does not support this functionality
  // rust actually indexes strings in three ways
  // The hindi word नमस्ते is stored as
  // 1. [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
  // 2. ['न', 'म', 'स', '्', 'त', 'े']
  // 3. ["न", "म", "स्", "ते"]
  // instead of returning indexes rust opts for the user to be more precise about what is wanted and a user can take a string slice from the original string, which returns a requested amount of characters from the 1. indexing type
  let hello = "Здравствуйте";
  let _s = &hello[0..4];
  // each of the above characters are 2 bytes so the returned value would be 'Зд'
  // requesting incorrect amount of bytes will cause rust to panic, ie. the characters are 2 bytes so if one would try &hello[0..1] it would not work
  
  // other ways of iteraing strings
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }
  // prints out characters in method 2.

  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
  // prints out characters in method 1.

  // rust doesn not provide out of the box way of iterating over characters in method 3. But crates.io has packages for that

  // rust opts for the complexity of using the correct handling of String the default for all rust programs
  // however but it prevents having to handle non ASCII characters
}