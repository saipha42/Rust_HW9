use std::{fs::File, io::Write};

use csv::{Reader, StringRecord};
use hw9::MyModule::{Circle, Layer, cal_avg_min_max_area};


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
    
    let avg_area = cal_avg_min_max_area(&layers);
    
    let mut html = String::new();
    html.push_str("<table>");
    html.push_str("<style>table, td, th {border: 1px solid #000000;\
        border-collapse : collapse;\
        padding : 6px;\
        }</style>");
    html.push_str("<table>\
        <tr>\
            <th>Layer Name </th>\
            <th>Average area</th>\
            <th>Maximum area</th>\
            <th>Minimum area</th>\
        </tr>\
        ");
    
    for layer in avg_area {
        let row = format!("<tr> <td>{}</td> \
        <td>{}</td>\
        <td>{}</td>\
        <td>{}</td>\
        </tr>", layer.name, layer.avg, layer.max, layer.min);
        html.push_str(&row);
    }
    html.push_str("</table>");

    let mut html_file = File::create("layers_avg_min_max.html").unwrap();
    html_file.write_all(html.as_bytes()).unwrap();

    println!("Successfully save layer Avg, Max and Min areas to file : layers_avg_min_max.html!");

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