

fn seggs(thing: Option<&str>) -> i32 {
    if thing.is_none() {
        println!("No thing given no seggs");
        return 1;
    }
    else {
        println!("thing is {}", thing.unwrap());
        return 0;
    }

}

fn main() {
    seggs(None);
    seggs(Some("hello!!"));
}