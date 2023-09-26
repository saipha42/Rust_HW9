

pub mod my_module {
    use std::f32::consts::PI;

    use rand::Rng;
    use rand::rngs::ThreadRng;

    #[derive(Debug)]
    pub struct Circle {
        pub x : i32,
        pub y : i32,
        pub r: i32
    }

    #[derive(Debug)]
    pub struct  Layer {
        pub name : String,
        pub color : String,
        pub objects : Vec<Circle>
    }


    pub fn gen_color()-> String {
        let mut rng = rand::thread_rng();
        let red = rng.gen_range(0..=255);
        let green = rng.gen_range(0..=255);
        let blue = rng.gen_range(0..=255);
        let alpha = rng.gen_range(0..=255);
        let color = format!("#{:02X}{:02X}{:02X}{:02X}", red, green, blue, alpha);

        color
    }

    pub fn gen_obj_layer_list(rng : &mut ThreadRng , n: u32)-> Vec<Layer> {
        
        let mut layers = Vec::new();

        for i in 0..n {

            let name = format!("Layer {}", i);
            let color = gen_color();
            let mut circles :Vec<Circle>= Vec::new();
            let num_circles = rng.gen_range(20..=50);

            for _ in 0..num_circles {

                let x = rng.gen_range(-100..100);
                let y = rng.gen_range(-100..100);
                let r = rng.gen_range(-10..10);

                circles.push(Circle { x, y, r })
            }

            let layer = Layer {
                name,
                color,
                objects : circles
            };

            layers.push(layer);
        }
        layers
    }

    #[cfg(test)]
    #[test]
    pub fn gen_obj_layer_list_test() {

        let layers = gen_obj_layer_list(&mut rand::thread_rng(), 2);

        for i in 0..layers.len() {

            assert_eq!(layers[i].name , format!("Layer {}", i));
            assert!(layers[i].color.len() > 0); //check that the color field exist
            for k in &layers[i].objects {
                assert!(k.x >=-100 && k.x <=100);
                assert!(k.y >=-100 && k.y <=100);
                assert!(k.r >=-10 && k.r <=10);
            }
        }
    }



    pub fn cal_average_area(layers : &Vec<Layer>)-> Vec<(String, f32)> {

        let mut layers_avg : Vec<(String, f32)> = Vec::new();

        for i in 0..layers.len() {
            let name = layers[i].name.clone();
            let mut avg = 0.0;


            for k in 0..layers[i].objects.len() {

                let r = layers[i].objects[k].r as f32;
                let area = PI* r*r;

                avg += area;

            }

            avg = avg / (layers[i].objects.len() as f32);

            layers_avg.push((name, avg))
        }


        layers_avg

    }

    #[cfg(test)]
    #[test]
    pub fn cal_average_area_test() {

        let layers = gen_obj_layer_list(&mut rand::thread_rng(), 2);
        let layers = cal_average_area(&layers);

        for i in 0..layers.len() {

            assert_eq!(layers[i].0 , format!("Layer {}", i));
            assert!(layers[i].1 > 0.0);
        }
    }

    pub struct Area {
        pub name : String,
        pub avg : f32,
        pub min: f32,
        pub max: f32
    }
    pub fn cal_avg_min_max_area(layers : &Vec<Layer>)-> Vec<Area> {
        
        let mut layers_areas: Vec<Area> = Vec::new();
        
        for layer in layers {

            let mut avg_area = 0.0;
            let mut max_area = 0.0;
            let r = layer.objects[0].r as f32;
            let mut min_area = PI * r * r;
            

            for i in &layer.objects {
                let radius = i.r as f32;
                let area = PI* radius * radius;
                avg_area += area;
                if area > max_area {
                    max_area = area;
                }
                if area < min_area {
                    min_area = area
                }

            }

            avg_area = avg_area / (layer.objects.len() as f32);

            layers_areas.push( Area {
                name : layer.name.to_string(),
                avg : avg_area,
                min : min_area,
                max: max_area
             })
        }

        layers_areas

    }


}