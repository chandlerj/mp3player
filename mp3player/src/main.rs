use std::env;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let args: Vec<String> = env::args().collect(); //program arguments
    dbg!(&args);

    // process_arguments(args);
    play_audio(String::from("/home/chandler/Music/Bitches.mp3"));

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

fn play_audio(audio_path: String){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let file = BufReader::new(File::open(audio_path).unwrap());

    let source = Decoder::new(file).unwrap();

    stream_handle.play_raw(source.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(5));
}