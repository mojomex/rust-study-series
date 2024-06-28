use rand::random;


fn ask_for_money() -> Option<i32> {
    let success = random::<bool>();
    if success {
        return Some(500)
    }

    None
}

fn roll_a_three() -> Result<i32, String> {
    let number = 1 + (random::<u8>() % 6);
    if number == 3 {
        return Ok(3)
    }

    Err(format!("Rolled a {number} 😭").to_owned())
}

fn main() {
    let numbers = [1,2,3];
    // println!("{}", numbers[5]); Causes a compiler error

    let numbers = vec![1,2,3,4,5,6,7];
    // println!("{}", numbers[10]);  // Causes a panic (program exit), NOT a segfault

    for i in 0..10 {
        // In case the option contains something, print x. Otherwise, print warning.
        match numbers.get(i) {
            Some(x) => println!("Got: {x}"),
            None => println!("Does not exist"),
        }
    }

    for _ in 0..10 {
        // Only run body of if-statement if the option contains something
        if let Some(result) = ask_for_money() {
            println!("{result}");
        }
    }

    // User is sure that there is no error. This is UNSAFE
    // let result = maybe_error().unwrap();  
    
    // The safe way to get the result is:
    let result = match roll_a_three() {
        Ok(x) => x,
        Err(msg) => {
            println!("{msg}"); 
            panic!();  // Did not get a 3, panic!
        },
    };

    println!("{result}")
}
