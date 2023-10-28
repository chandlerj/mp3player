use std::env;
use std::sync::mpsc;
use std::thread;
use crate::audio_engine::AudioPlayer;
use futures::executor::block_on;
mod audio_engine;
mod user_interface;

fn main() {
    block_on(async_main());
}
   

async fn async_main(){
    
    let args: Vec<String> = env::args().collect(); //program arguments
    let mut audio_player: AudioPlayer = AudioPlayer::initialize_engine();
//    let file_name: String = process_arguments(args);
    let file_name = String::from("/home/chandler/Desktop/hazarddutypay.mp3");
    
    let player = audio_player.play_audio(&file_name); 
    player.await;
    //user_interface::display_track_metadata(&audio_player);}
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


