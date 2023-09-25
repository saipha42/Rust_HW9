use hw9::MyModule::gen_obj_layer_list;
use csv::WriterBuilder;

fn main() {

    let mut rng = rand::thread_rng();
    let layers = gen_obj_layer_list(&mut rng, 10);

    let mut csv_writer = WriterBuilder::new()
                        
                        .has_headers(true).from_path("saved_layers.csv").unwrap();
    
    csv_writer.write_record(["name", "color", "points(x, y, r)"]).unwrap();
    
    for layer in &layers {
        let name = layer.name.clone();
        let color = layer.color.clone();
        let mut points = String::new();

        for pt in &layer.objects {
            points.push_str(format!(" ({}, {}, {}),", pt.x, pt.y, pt.r).as_str());
        }
        points.pop();

        csv_writer.serialize([[name], [color], [points]]).unwrap();
    }

    csv_writer.flush().unwrap();
    println!("Successfully save layers to file : saved_layers.csv");
    
}