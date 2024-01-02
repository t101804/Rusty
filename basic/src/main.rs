use std::thread::sleep;

// return an int and sleep for 1 second
fn do_something() -> i32 {
    let mut x = 0;
    let mut y = false;
    x += 1;
    y = !y;
    println!("{} {}", x, y);
    sleep(std::time::Duration::from_secs(1));
    5
}

// take an argument and return string with condition
fn parse(x: &str) -> &str {
    if x == "hi" {
        return "hi too";
    } else if x == "learn" {
        return "cool";
    }
    return "hello";
}

// loop and while
fn this(method: i32, max_value: i32) {
    let mut i = 0;
    if method == 1 {
        while i <= max_value {
            println!("print {} and will done if already {}", i, max_value);
            i += 1;
            sleep(std::time::Duration::from_secs(1))
        }
    } else if method == 2 {
        let result = 'mainLoop: loop {
            let mut j = max_value;
            let inner = i;
            'printLoop: loop {
                print!("X");
                j -= 1;
                // println!("{j}");
                // std::process::exit(1);
                if j < inner {
                    break 'printLoop;
                }
            }
            print!("\n");
            if i >= max_value {
                break 'mainLoop i;
            }
            i += 1;
            // println!("printing {}", i);
        };
        println!("result: {result}");
    } else if method == 3 {
        'forloop: for i in 0..=max_value {
            if i == 3 {
                break 'forloop;
            }
            println!("please be ready in {i}")
        }
        'getname: for name in ["jason", "haxor", "pajar", "ilham"] {
            println!("{name}");
            if name == "ilham" {
                println!("ditemukan orang autis {name}");
                break 'getname;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    do_something();
    parse("hi");
    this(3, 5);
    println!("test xor");
    Ok(())
}
