fn main() {
    println!("Welcome!");
    fizz_buzz();
}

fn fizz_buzz() {
    let mut count: i32 = 0;

    for n in 1..=301 {
        match n {
            n if n % 15 == 0 => {
                println!("FizzBuzz");
                count += 1;
            }
            n if n % 3 == 0 => println!("Fizz"),
            n if n % 5 == 0 => println!("Buzz"),
            _ => (),
        }
    }
    println!("FizzBuzz count: {}", count);
}
