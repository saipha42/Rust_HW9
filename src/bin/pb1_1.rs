use hw9::my_module::gen_obj_layer_list;

fn main() {

    let mut rng = rand::thread_rng();

    let layers = gen_obj_layer_list(&mut rng, 2);

    println!("List of layers : {:#?}", layers);

}