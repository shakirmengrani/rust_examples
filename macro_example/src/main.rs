macro_rules! log {
    ($e:expr) => {
        println!("log builder says: {:?}", $e)
    };
}

macro_rules! sqrt {
    ($x:expr) => ( $x * $x );
}

macro_rules! matching_macro {
    (x => $e:expr) => (log!(format!("mode X: {}", $e)));
    (y => $e:expr) => (log!(format!("mode Y: {}", $e)));
}

macro_rules! yolo_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                let mut _sqt = sqrt!($x);
                temp_vec.push(_sqt);
            )*
            temp_vec
        }
    };
}

fn get_custom_type<T>(x: T) -> T{
    return x;
}

fn main() {
    log!([1,2,3].iter_mut());
    log!(format!("sqrt of {:?} is {}", 4, sqrt!(4)));
    matching_macro!(x => 3);
    matching_macro!(y => 6);
    log!(format!("yolo: {:?}", yolo_vec![1,2,3]));
    log!(get_custom_type("shakir"));
}
