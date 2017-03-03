   /// comment
fn main() {
    //let x = 5;
    let (x,y) = (1,2);
    let x1: i32 = 5;
    println!("{}",x1);
    println!("hello world {} {}",x,y);    


    let mut x2 = 2;
     println!("x2 is {} before modify",x2);
    x2 =18;
    println!("x2 is {} after",x2);
    println!("x1 + x2 ={}" , sum(x1, x2));


    let f:fn() -> i32;
    f = get;
    x2 = f();
    println!("{}",x2);


    let x3 = false;
    let x4: bool = false;
    println!("{}",x3 ==x4);


    let arr1 = [1,2,3];
    let mut arr2 = [0;20];
    let arr3 = &arr1[..];
    arr2[0] = 3;
    println!("arr1 len is: {}",arr1.len());
    println!("arr2 len is: {}",arr3.len());
    println!("{}", arr2[0]);



    let x4 = (1, "2d",34);
    println!("{}",x4.1);

    //  if 
    if x1 == 5 {
        println!("x1 is {}",x1);
    }

    let y46 = if x1 == 3 {
        10
    } else {
        23
    };
    println!("y is equal {}",y46);


    let mut first = 1;
    let mut second = 1;
    let mut sum = 0;
    let mut i = 0;
    while i < 10 {
        sum = first + second;
        let temp = first + second;
        first = second ;
        second = temp;
        i = i + 1;
    }

    println!("fibolacial is {}",sum);

    i = 0;

    loop {
        if i >= 5 {
            break;
        }
        i = i + 1;
        println!("loop is {}",i);
    }

    for i in 0..10 {
        print!("{} ", i);
    }
    println!();

    for (index,value) in (0..3).enumerate() {
        println!("key {}, value {} ", index,value);
    }


    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }


}

fn sum (x: i32, y: i32) -> i32  {
     x + y
}

fn get() ->  i32{
    return 3;
}



