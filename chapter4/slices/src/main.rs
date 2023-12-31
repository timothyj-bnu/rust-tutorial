fn main() {

    let mut s = String::from("hello world");

    let slice = &s[0..=5]; 
    let slice = &s[..2];
    let slice = &s[3..];

    println!("{}", slice);

    let first = first_word(&s);

    // s.clear();

    println!("{}", first);

     let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial
    // or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which
    // are equivalent to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals,
    // whether partial or whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    let a: [i32;5] = [1, 2, 3, 4, 5];

    
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{ // if find a space
            return &s[0..i];
        }
    }

    return &s[..];
}
