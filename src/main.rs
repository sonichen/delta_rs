 

use delta_rs::delta_rs::{ get_content, Deltas};

fn main() {
    // Write data
    let mut deltas=Deltas::create("Hello, I am a rust developer.");
    
    // First modify
    deltas.modify("Hello, I am a Java developer.");

    // Second modify
    deltas.modify("Hello, I am a Python developer.");

    // Third modify
    deltas.modify("Hello, I am a Go developer.");

    // print the data block and the content
    for item in &deltas.deltas {
        println!("{}", get_content(item.id, deltas.deltas.clone()));
        println!("{:?}\n", item);
    }
    
}