fn first_word(s: &String) -> &String {
  let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return s[0..i];
        }
    }

    s
}

pub fn slices() {
  // A String / Vec OWNS its data and is GROWABLE
  // String internally (simplified):
  //   pointer: *u8
  //   size: isize
  //   capacity: isize

  // A slice REFERENCES data owned by someone else
  // Slice internally:
  //   pointer: *u8
  //   size: isize

  // Slices exist for all CONTIGUOUS array-like types:
  let str_owned = String::from("Hi!");
  let str_slice = &str_owned[..];

  let vec_owned = vec![1,2,3];
  let vec_slice = &vec_owned[..];
}