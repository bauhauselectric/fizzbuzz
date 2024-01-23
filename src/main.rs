fn main() {
    for i in 1..101 {
        if i % 3 ==0 && i % 5 == 0 {
            println!("{:3} FizzBuuzz", i);
        } else if i % 3 == 0 {
            println!("{:3} Fizz", i);
        } else if i % 5 == 0 {
            println!("{:3} Buzz", i);
        } else {
            println!("{:3}", i);
        }
    };
}
