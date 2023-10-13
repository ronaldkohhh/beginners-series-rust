fn main() {
    if 1 == 2 {
        println!("Math is broken!");
    } else {
        println!("Everything is fine!");
    }

    let formal = true;
    let greeting = if formal {
        println!("Good evening!");
    } else {
        println!("Hey friend!");
    };

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("Number is not divisible by 3 or 4");
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };
}
