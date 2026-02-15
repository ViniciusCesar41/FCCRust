fn main (){
    println!("Hello world!");

    let mut x: i32 = 5;

    x += 1;

    assert_eq!(x, 6);
    println!("sucess!");

    define_y();
}

fn define_y(){
    let y: &str = "hello";
    println!("{} world!", y);
}