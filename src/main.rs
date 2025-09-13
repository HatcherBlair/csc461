fn print1_through10() {
    for i in 1..11 {
        println!("{}", i);
    }
}

fn sum1_through10() -> i32 {
    let mut tot = 0;
    for i in 1..10 {
        tot += i;
    }
    tot
}

fn main() {
    println!("Hello, world!");
    print1_through10();
    println!("{}", sum1_through10());
}
