// #[allow(unused_variables)]

fn main (){
    println!("Hello world!");

    let mut x: i32 = 5;

    x += 1;

    assert_eq!(x, 6);
    println!("sucess!");

    define_y();

    // let x = 1;

    tuple_test();
}

fn define_y(){
    let y: &str = "hello";
    println!("{} world!", y);
}

fn tuple_test(){
    let (mut x, y) = (1,2);
    x += 2;

    assert_eq!(x,3);
    assert_eq!(y,2);
    println!("tuplesuccess!");
    
}