use hw9::MyModule::{gen_obj_layer_list, cal_average_area};





fn main() {
    

    let mut rng = rand::thread_rng();

    let layers = gen_obj_layer_list(&mut rng, 2);

    let avg_layers = cal_average_area(&layers);
    println!("List of Average Areas : {:#?}", avg_layers);
}