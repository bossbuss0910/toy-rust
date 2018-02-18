fn fizzbuzz(n: usize) {
    for i in 0.. n{
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else { println!("Buzz"); }
    }
}

fn main() {
    fizzbuzz(20);
}
