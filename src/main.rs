fn main() {
    println!("{}", fizz_buzz(1));
    println!("{}", fizz_buzz(3));
    println!("{}", fizz_buzz(5));
    println!("{}", fizz_buzz(15));
}

fn fizz_buzz(number: u8) -> String {
    match number {
        is_fizz => "fizz".to_string();
    }
    if is_fizz(number) && is_buzz(number) {
        return "FizzBuzz".to_string();
    }
    if is_fizz(number) {
        return "fizz".to_string();
    }
    if is_buzz(number) {
        return "buzz".to_string();
    }
    return number.to_string();
}

fn is_fizz(number: u8) -> bool {
    return number % 3 == 0;
}

fn is_buzz(number: u8) -> bool {
    return number % 5 == 0;
}