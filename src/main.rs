 

use track_rs::track_rs::{ get_content, Track};

fn main() {
    // Write data
    let mut track=Track::create("Hello, I am a rust developer.");
    
    // First modify
    track.modify("Hello, I am a Java developer.");

    // Second modify
    track.modify("Hello, I am a Python developer.");

    // Third modify
    track.modify("Hello, I am a Go developer.");

    // print the data block and the content
    for item in &track.track {
        println!("The text in {} time is '{}'",item.id, get_content(item.id, track.track.clone()));
        println!("{:?}\n", item);
    }
    
}