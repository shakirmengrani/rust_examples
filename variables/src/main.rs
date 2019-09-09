fn main() {
    println!("Hello, world! {}", "shakir");
    let mut x = 5; // if "mut" not present then twice immutable error will appears
    println!("my var {0} x {1} = {2}", x, x, x*x);
    x = 6;
    println!("my var {0} x {1} = {2}", x, x, x*x);
    let _guess: u32 = "42".parse().expect("Not a number");

    let _a = 5;
    let b = {
        let _a = 1;
        _a + 1
    };
    println!("{0} {1}", b, help());
    println!("\n");
    for num in (1..5).rev().rev(){
        println!("{}",num);
    }
    println!("\n");
    for num in [1,2,3,4].iter() {
        println!("{}",num);
    }
    let num: String = if _a == 5 {
        String::from("got it")
    } else {
        String::from("didn't get")
    };

    println!("{} {}", num, help())

}

fn help() -> String {
    return String::from("Shakir");
}