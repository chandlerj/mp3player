
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, OutputStreamHandle};
pub fn play_audio(audio_path: String){
    let mut _stream: OutputStream;
    let stream_handle: OutputStreamHandle;
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let outputstream = OutputStream::try_default();
    match outputstream{
        Ok((_s, s_h)) => {
            let x = Some(2);
            _stream = _s; 
            stream_handle = s_h;
            let file = BufReader::new(open_file(audio_path));

            let source = Decoder::new(file).unwrap();

            stream_handle.play_raw(source.convert_samples()).expect("Could not play audio file");

            std::thread::sleep(std::time::Duration::from_secs(5));

        }
        Err(e) => {
            println!("Unable to initialize audio device: {e}")
        }
    }
    
}

fn open_file(audio_path: String) -> File {
    let file = File::open(audio_path);
    match file {
       Ok(processed_file) => {
            return processed_file;
       }
       Err(e) => {panic!("Unable to process file {e}")}
    };
}