const STARTING_MISSILES: i32 =  4;
const READY_AMOUNT: i32 = 2;


fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles); //part 1

    missiles = missiles - ready; //part 2
    println!("{} missiles left", missiles); //part 2
}
