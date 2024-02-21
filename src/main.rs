fn main() {
    define_x();
    assert_x();
    multi_scope();
    destructuring();
}

fn assert_x() {
    let x: i32 = 5;
    // let y: i32;

    assert_eq!(x,5);
    println!("Success!");

    let mut z = 1;
    z = z + 2; // 3 

    print!("z is now {}", z)
    // Most of the time type declarations aren't necessary
}

fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x)
}

fn multi_scope() {
    let x:i32 = 7;

    {
        let x = 12;
        assert_eq!(x, 12);
        println!("success");
    }

    //println vs print? 

    println!("x is still 7 in outer scope see:: {}", x)
}

fn destructuring() {
    // destructuring a tuple of 2 i8 integers
    let (mut x, y) = (1 , 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    print!("Success!");

    // use .. to ignore values
    let (a,b);

    (a, ..) = (3, 4);
    [.., b] = [1, 2];

    assert_eq!([a, b], [3, 2]);
    println!("Destructuring successful")
}

let wanda: &str = "Dec 10";