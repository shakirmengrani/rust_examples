fn main() {
    let mut x: i32 = 1;
    loop {
        println!("Print line # {}", x);
        x += 1;
        if x > 20 {
            break;
        }
    }

    let mut count: i32 = 0;
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        }else if count % 3 == 0 {
            println!("fizz");
        }else if count % 5 == 0{
            println!("buzz");
        }else{
            println!("{}", count);
        }
        count += 1; 
    }
    for x in ["Shakir", "World"].iter() {
        println!("{}", x);
    }

    for (x, y) in (5..11).enumerate(){
        println!("{} {}", x, y);
    }
    
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // continues the loop over x
            if y % 2 == 0 { continue 'inner; } // continues the loop over y
            println!("inner: {}, outer: {}", x, y);
        }
    }
}
