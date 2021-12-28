use hello::greet;

fn main() {
    println!("Hello, world!");
    let _bunnies: i32 = 4;
    let (_bunnies_2, _carrots) = (4, 5);

    let _x = 5;
    {
        let _x = 99;
    }
    greet();
}
