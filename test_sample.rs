fn simple_function() {
    println!("Hello world");
}

fn function_with_if(x: i32) -> i32 {
    if x > 0 {
        x + 1
    } else {
        0
    }
}

fn complex_function(a: i32, b: i32) -> i32 {
    if a > b {
        if a > 10 {
            a * 2
        } else {
            a + b
        }
    } else {
        if b > 10 {
            b * 3
        } else {
            a - b
        }
    }
}