// since we have both unused methods and unused
// variables as this is a tutorial we can ignore these warnings
#![allow(unused_variables)]
#![allow(dead_code)]


fn find_distance(){
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let kcle_latitude_degrees:f64 = 41.4075;
    let kcle_longitude_degrees:f64 = -81.851111;
    let kslc_latitude_degrees:f64 = 40.7861;
    let kslc_longitude_degrees:f64 = -111.9822;

    let kcle_latitude_raidans = kcle_latitude_degrees.to_radians();
    let kslc_latitude_radians = kslc_latitude_degrees.to_radians();

    let delta_latitude = (kcle_latitude_degrees - kslc_latitude_degrees).to_radians();
    let delta_longitude = (kcle_longitude_degrees - kslc_longitude_degrees).to_radians();

    let inner_central_angle = f64::powi((delta_latitude/2.0).sin(),2)
        + kcle_latitude_raidans.cos()
        * kslc_latitude_radians.cos()
        * f64::powi((delta_longitude/2.0).sin(),2);

    let central_angle = 2.0 * inner_central_angle.sqrt().asin();
    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
    println!("the distance between the two points is {:.1}",distance);
}