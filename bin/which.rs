use find_in_path::prelude::*;

fn main() {
    let args = std::env::args();

    for (index, arg) in args.into_iter().enumerate() {
        let path = arg.find_in_path();
        if path.is_some() && index != 0 {
            println!("{}", path.unwrap().to_str().unwrap());
        }
    }
}