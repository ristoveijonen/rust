pub fn hash_maps() {
  // HashMaps are key value collections
  // HashMap needs to be imported from the standard collection, unlike vectors and strings
  use std::collections::HashMap;
  let mut scores = HashMap::new();
  // HashMaps have both keys and values typed
  // All the keys of a HashMap must be of same type as well as the values
  // here we have a HashMap with type string keys and i32 values
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  println!("teams and their scores are: {:?}", scores);

  // Another way of creating the above HashMap
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![15, 17];
  // Make a list of tuples that have values from teams paired with values from initial_scores and then collects those tuples as a HashMap
  // The datastructure is specified by <_, _>, but rust infers the types from the values
  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
  println!("teams and their scores are: {:?}", scores);

  // Types that have the 'copy' trait like i32, are copied into HashMaps, but other types like String are are owned
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");
  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  println!("{:?}", map);
  // field_name and field_value are no longer
  // values can be gotten from a HashMap with the key
  let team_name = String::from("Blue");
  let score = scores.get(&team_name);
  println!("{:?}", score);

  // HashMaps can be iterated like vectors are
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  // A key can only hold one value at a time
  // By default when insterting data to the same key twice, the data will be overwritten
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);
  println!("{:?}", scores);

  // using the entry api we can insert data if none already present and disregard the insert if key already found
  // entry represents a value that might or might not exist
  // the ir_insert() for entry either uses the value retrieved with the provided key or applies the value given as the value for the provided key
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);
  println!("{:?}", scores);

  // we can also update the value in HashMap based on the value already stored
  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    // count is a reference to the value in map with the word key
    // if a key doesn't exist or_insert creates one with value 0
    // or_insert returns a mutable reference to the value
    let count = map.entry(word).or_insert(0);
    // here we dereference the count with *, meaning the reference will end after the loop and all the changes are safe and allowed by rust
    *count += 1;
  }
  println!("{:?}", map);

  // rust uses by default a slower version of hashing that provides secuirty against DoS attacks. This function can be swapped for a faster one if necessary
}
