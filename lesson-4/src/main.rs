#[allow(unused_variables)]
#[allow(dead_code)]

fn space()
{
    println!("",);
}

fn add(_x: i32, _y: i32)->i32
{
    return _x+_y; 
}

fn square(number: i32)->i32
{
    let conclusion: i32 = number*number;
    return conclusion;
}

fn divide(k: i32, g: i32)->i32
{
    let conclusion: i32 = k/g;
    return conclusion;
}

fn to(n: i32, m: i32)->i32
{
    let mut conclusion: i32 = n;
    
    for  _ in  1..m {
        conclusion *= n;
    }


    return conclusion;
}


fn main()
{
    let mut result: i32 = 0;

    for  _x in  1..=32 {
        result += 32;
    }

    println!("{}",result); // 1024

    space();

    println!("{}",add(512,512)); // 1024

    space();

    println!("{}",square(32)); // 1024

    space();

    println!("{}",divide(2048,2)); // 1024

    space();
    
    println!("{}",to(2,10)); // 1024
}