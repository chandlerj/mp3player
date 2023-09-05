use std::env;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, OutputStreamHandle};



fn main() {
    let args: Vec<String> = env::args().collect(); //program arguments
    dbg!(&args);

    let file_name: String = process_arguments(args);
    play_audio(file_name);

}


fn process_arguments(args: Vec<String>) -> String {
    if args.len() <= 1{
        panic!("Insufficient number of arguments. exiting.");
    }

    if args.len() > 1{
        let file_path = &args[1];
        println!("playing file {file_path}");
        return file_path.to_string();
    }

    else{
        return String::from("error");
    }
    
}


fn play_audio(audio_path: String){
    let mut _stream: OutputStream;
    let stream_handle: OutputStreamHandle;
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let outputstream = OutputStream::try_default();
    match outputstream{
        Ok((_s, s_h)) => {
            _stream = _s; 
            stream_handle = s_h;
            let file = BufReader::new(File::open(audio_path).unwrap());

            let source = Decoder::new(file).unwrap();

            stream_handle.play_raw(source.convert_samples()).expect("Could not play audio file");

            std::thread::sleep(std::time::Duration::from_secs(5));

        }
        Err(e) => {
            println!("Unable to initialize audio device: {e}")
        }
    }
    
}