use delta_store::delta_store::{Delta, get_full_data};

fn main() {
    let mut detlas = Delta::init("ABCDEFGHIJKLMNOPQRSTUVWXYJ");
    detlas = Delta::add("1BCDEFGHIJKLMNOPQRSTUVWXY1", detlas, false);
    detlas = Delta::add("ABCDEFGHIJKLMNOPQRSTUVWXY8", detlas, false);
    detlas = Delta::add("ABCDEFG5IJKLMN7PQRSTUVWXYJ", detlas, false);
    detlas = Delta::add("12121", detlas, true);

    for item in &detlas {
        println!("{}", get_full_data(item.id, detlas.clone()));
        println!("{:?}\n", item);

    }
    
}