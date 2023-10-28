use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, OutputStreamHandle, Sink};
use std::time::Duration;
use std::thread;

pub struct AudioPlayer {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    pub track_length: Duration,
    pub file_path: String,
    pub sink: Sink,

}
impl AudioPlayer{

    // essentially the constructor for audio player
    pub fn initialize_engine() -> AudioPlayer{
        let outputstream = OutputStream::try_default();
        match outputstream{
            Ok((_s, s_h)) => {
                let stream = _s; 
                let stream_handle = s_h;
                let sink = Sink::try_new(&stream_handle).unwrap();
                let track_length: Duration = Duration::new(0,0);
                let file_path = String::from("");
                AudioPlayer{stream, stream_handle, sink, track_length, file_path}
            }
            Err(e) => {
                panic!("Unable to initialize audio device: {e}")
            }
        }
    }

    // plays file from specified path
    pub async fn play_audio(&mut self, audio_path: &String){
        let file = BufReader::new(AudioPlayer::open_file(audio_path.clone())); 
        let source = Decoder::new(file).unwrap();
        //self.track_length = source.total_duration().unwrap();
        self.file_path = audio_path.clone();
        self.sink.append(source);
        self.sink.sleep_until_end();
        
    }
    

    // attempts to open file from specified path
    // TODO: Make it so if a directory is opened, every audio file in path gets appended to sink
    fn open_file(audio_path: String) -> File {
        let file = File::open(audio_path);
        match file {
           Ok(processed_file) => {
                return processed_file;
           }
           Err(e) => {panic!("Unable to process file {e}")}
        };
    }

}


unsafe impl Sync for AudioPlayer {}  
unsafe impl Send for AudioPlayer {}
