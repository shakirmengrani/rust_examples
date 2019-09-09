fn main() {
    println!("Hello, world!");
    let s1 = String::from("Shakir");
    let s2 = s1.clone(); // to makes a reference content copy 
    println!("{} {}",s1, s2);
    take_ownership(s2);
    // println!("{}", s2); // make compile time error about allocation of memory or move out from the scope
    let x = 5;
    make_copies(x);
    let proc_back = takes_process_and_gives_back_ownership(s1);
    println!("procBack {}", proc_back);
    let (s, len) = calc_len(proc_back);
    println!("str {} & size {}", s, len);
}

fn take_ownership(str: String) {
    println!("func Owns {}", str);
}

fn make_copies(x: i32){
    println!("x: {}", x)
}

fn takes_process_and_gives_back_ownership(mut s: String) -> String{
    s.push_str(", Hi");
    s
}

fn calc_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}