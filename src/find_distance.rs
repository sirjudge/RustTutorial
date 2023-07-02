// since we have both unused methods and unused
// variables as this is a tutorial we can ignore these warnings
#![allow(unused_variables)]
#![allow(dead_code)]


pub fn main_run(){
    //find_distance();
    distance_routing();
}

fn distance_routing(){
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;
    let routes = [
        ("kcle",41.4075,-81.851111),
        ("GITJ",41.7680,-86.604049),
        ("FOD",42.61110,-94.24980),
        ("PUDVV",41.54270,-109.99000),
        ("WEGEM",41.44560,-109.34200),
        ("KSLC",40.7861,-111.9822)
    ];

    let mut totatl_distance = 0.0;
    let mut previous_waypoint: Option<(&str,f64,f64)> = None;

    for waypoint in routes.iter(){
        match previous_waypoint{
            None => {
                previous_waypoint = Option::from(waypoint.clone());
                continue;
            }
            Some(previous_waypoint_value) =>{
                let  distance = find_distance(waypoint.clone(),previous_waypoint_value);
            }
        }
    }

}

fn find_distance(coords1 : (&str, f64,f64), coords2:  (&str,f64,f64))-> f64 {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    // set distance latitude and longitude from coords touples
    let distance1_latitude_degrees:f64 = coords1.1;
    let distance_1_longitude_degrees:f64 = coords1.2;
    let distance2_latitude_degrees:f64 = coords2.1;
    let distance2_longitude_degrees:f64 = coords2.2;

    // get radians from dregrees and find the deltas
    let distance1_latitude_raidans = distance1_latitude_degrees.to_radians();
    let distance2_latitude_radians = distance2_latitude_degrees.to_radians();
    let delta_latitude = (distance1_latitude_degrees - distance2_latitude_degrees).to_radians();
    let delta_longitude = (distance_1_longitude_degrees - distance2_longitude_degrees).to_radians();

    // Find central angle and multiply earths radius and that gives us the final distance because
    // magic math
    let inner_central_angle = f64::powi((delta_latitude/2.0).sin(),2)
        + distance1_latitude_raidans.cos()
        * distance2_latitude_radians.cos()
        * f64::powi((delta_longitude/2.0).sin(),2);
    let central_angle = 2.0 * inner_central_angle.sqrt().asin();
    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

    println!("distance between {} and {} is {}",coords1.0,coords2.0,distance);

   return distance;
}