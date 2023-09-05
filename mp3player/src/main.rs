use std::env;
mod audio_engine;


fn main() {
    let args: Vec<String> = env::args().collect(); //program arguments
    dbg!(&args);

    let file_name: String = process_arguments(args);
    audio_engine::play_audio(file_name);

}


fn process_arguments(args: Vec<String>) -> String {
    match args.len(){
        0 => panic!("Insufficient number of arguments. exiting."),
        1 => panic!("Insufficient number of arguments. exiting."),
        2 => {
            let file_path = &args[1];
            println!("playing file {file_path}");
            return file_path.to_string();
        }
        _ => panic!("unable to process arguments."),

    }
    
}


