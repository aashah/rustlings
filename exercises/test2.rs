// test2.rs
// This is a test for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `Strings`, some are `&strs`. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // string literal
    string("red".to_string()); // literal converted to string
    (String::from("hi")); // using From trait
    string("rust is fun!".to_owned()); // string() function takes ownership of "arg"
    string("nice weather".into()); // into() takes ownership and converts type. in this case, input was string slice
    string(format!("Interpolation {}", "Station")); // modifying string
    string_slice(&String::from("abc")[0..1]); // taking a portion of a String
    string_slice("  hello there ".trim()); // we return a piece of string literal
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // modifying String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // modifying String
}
