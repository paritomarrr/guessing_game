fn main() {
    // var
    // let mut x = 5;
    // println!("The val of x is, {x}");

    // // constants
    // const THREE_HOURS_IN_SEC: u32 = 60 * 60 * 3;
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The valye of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

}
