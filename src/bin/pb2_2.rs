

use csv::{Reader, StringRecord};
use hw9::MyModule::{Circle, Layer, cal_average_area};


fn main() {

    let mut csv_layers = Reader::from_path("./layers.csv").unwrap();
    let mut layers : Vec<Layer> = Vec::new();
    for record in csv_layers.records() {
        
        if let Ok(layer) = record {
            
            let name = &layer[0];
            let color = &layer[1];
            let objects = interpret_csv(&layer);
            layers.push(
                Layer { 
                    name : name.to_string(), 
                    color : color.to_string(), 
                    objects
                }
            );

        }
    }
    
    let avg_area = cal_average_area(&layers);
    let mut csv_writer = csv::Writer::from_path("layers_avg.csv").unwrap();
    csv_writer.write_record(["Layer", "Avg_Area"]).unwrap();
    for i in avg_area {
        csv_writer.write_record([i.0, i.1.to_string()]).unwrap();
    }
    csv_writer.flush().unwrap();

    println!("Successfully save layer average areas to file: layers_avg.csv !");

}



fn interpret_csv(layer: &StringRecord) -> Vec<Circle> {

    let circles = &layer[2];
    let mut objects : Vec<Circle>= Vec::new();
    for pt in circles.split(")"){
        
        if pt.len() <=1 {
            continue;
        }
        
        let point = pt.replace(" ", "");
        let point = point.replace("(", "");
        let point = point[1..].to_string();
        let point : Vec<_>= point.split(",").map(| p| {
            let point = p.parse::<i32>().unwrap();
            point
        } ).collect();
        objects.push(Circle { 
            x: point[0], y: point[1], r: point[2] });
    }

    objects
}