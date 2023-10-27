 

use delta_rs::delta_rs::{ get_full_data, Deltas};

fn main() {
    let mut deltas=Deltas::create("ABCDEFGHIJKLMNOPQRSTUVWXYJ");
    deltas.add("1BCDEFGHIJKLMNOPQRSTUVWXY1");
    deltas.add("AB9DEFGHIJKLMNOPQRSTUVWXY8");
    deltas.add("AB9DEFGHIJ4121NOPQRSTUVWXY8");
    deltas.add("AB9DEFGHI121OPQRSTUVWXY8");
    deltas.add("AB9D2FGH421MNOPQRSTUVWXY8");
    deltas.add("AB9D2FGHIJKLMNOPQRSTUVWXY8");
    deltas.add("AB9D1LMNOPQRSTUVWXY8");


    println!("{:#?}",deltas);
    for item in &deltas.deltas {
        println!("{}", get_full_data(item.id, deltas.deltas.clone()));
        println!("{:?}\n", item);

    }
    
}