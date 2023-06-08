use std::env;
fn main() {
    let args: Vec<String> = env::args().collect(); //program arguments
    dbg!(&args);

    process_arguments(args);


}


fn process_arguments(args: Vec<String>){
    if args.len() <= 1{
        println!("Insufficient number of arguments. exiting.");
    }

    if args.len() > 1{
        let file = &args[1];
        println!("playing file {file}")
    }
}