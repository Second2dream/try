fn main() {
    let mut s = String::from("hello");

    println!("{s}");

    let s2 = give_ownerships(s);

    println!("{s2}");

    let s3 = give_back(s2);

    println!("{s3}");
}

fn give_ownerships(mut s_2:String) -> String {
    s_2.push_str(",world!");

    println!("{s_2}");

    s_2
}

fn give_back(s2_2:String) -> String {
    s2_2
}