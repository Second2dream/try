fn main() {
    let mut a = 1;

    println!("{a}");

    let mut b = 0;

    for i in (1..10){

        println!("{}",a);

        b = a - b;

        a += b;

    }
}
