fn main() {
    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y);
    }
    println!("{}", x);


    //sample immutable convert to mutable
    // let mut x = 4;
    // let x = x;
}
