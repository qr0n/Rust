use::std::io;

struct MenuItem {
    name : String,
    price : i32,
}

struct TupleStruct(String, i32);

fn main() {
    let _test: MenuItem = MenuItem {
        name : String::from("hello world"),
        price : 100
    };
    println!("{}", _test.name);
    println!("{}", _test.price);

    let _test2: TupleStruct = TupleStruct(String::from("hello world"), 100);
}