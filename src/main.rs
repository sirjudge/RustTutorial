fn main() {
    //tuples();
    strings();
}

fn tuples() {
    let location = ("KCLE", 41.4094069,-81.8546911);
    let (name,latitude,longitude) = location;
    println!("locationName:{name}, latitude:{latitude}, longitude:{longitude}");
}

fn strings(){
    let person_name_string = String::from("Nico judge");
    let person_name_slice = &person_name_string;
    let person_name_slice2 = person_name_slice.as_str();

}
