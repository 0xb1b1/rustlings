// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.



fn main() {
    let optional_word = Some(String::from("rustlings"));  // : Option<String> = None;
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(i) = optional_word {
        println!("The word is: {}", i);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(i) = optional_integers_vec.pop() {
        println!("current value: {}", i.unwrap());
    }  // Halts when .pop() returns None
}

// The `if let` construct reads: "if `let` destructures `number` into
// `Some(i)`, evaluate the block (`{}`).
