use std::{io::Read, thread::sleep};
mod io;
mod mstr;
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

fn axray() {
    let mut i = 0;
    // let mixed = [1, false, "person"];
    let mut nums: [i32; 3] = [1; 3];
    let mut person: [&str; 3] = ["haxor", "leet", "skid"];
    nums[1] = 2;
    nums[2] = 3;
    println!("nums {:?}", nums);
    person[0] = "rep";
    println!("total rank: {}\nrank {:?}", person.len(), person);
    // for i in 0..person.len() {
    //     println!("i:{} person:{}", i, person[i])
    // }
    // loop {
    //     if i < person.len() {
    //         println!("i: {}, person:{}", i, person[i]);
    //         i += 1;
    //     } else {
    //         break;
    //     }
    //     // if i == names.len() {
    //     //     break;
    //     // }
    //     // println!("i: {}, person:{}", i, person[i]);
    //     // i += 1;
    // }
    // while i < person.len() {
    //     println!("{}", person[i]);
    //     i += 1;
    // }

    for (i, name) in person.iter().enumerate() {
        println!("i:{} person:{}", i, name);
    }
}

fn playing() {
    let mut tuple_a = ("jason", 27, ["racing", "working out"], true);
    tuple_a.0 = "rep";
    println!("{tuple_a:?}");
    let mut v = vec!["google.com", "kpu.go.id", "velixs.com"];
    v.push("rust.com");
    // for (i, site) in v.iter().enumerate() {
    //     println!("{i} : {site}");
    // }
    for i in &v {
        println!("{i}");
    }
    for i in 0..v.len() {
        println!("{}", v[i])
    }
}

fn stakeinpt() -> String {
    println!("input ur name : ");
    let mut msg = String::new();
    let getinpt = std::io::stdin();
    let read = getinpt.read_line(&mut msg);
    if read.is_err() {
        println!("err! : {:?}", read.err());
    }
    return msg;
}

mod utilities {
    pub mod password {
        use std::{result, str::Bytes};

        pub fn bcryptor(text: &str) -> String {
            let result = bcrypt::hash(text, bcrypt::DEFAULT_COST).unwrap();
            return result;
        }
        pub fn verifybcrypte(plain: &str, text: &str) -> bool {
            bcrypt::verify(plain, text).unwrap()
        }
        pub fn cryptx(text: &str, salt: &str) -> String {
            let x: Vec<_> = text.chars().collect();

            let mut result = std::string::String::new();
            for (i, chars) in text.chars().into_iter().enumerate() {
                println!("{}", x[i]);
                for (si, s) in salt.chars().into_iter().enumerate() {
                    let encrypted_char = chars.to_string() + &s.to_string() + &x[i].to_string();
                    result.push_str(&encrypted_char)
                }
            }
            result
        }
        pub fn decrypt(encrypted_text: &str, salt: &str) -> String {
            let mut result = String::new();
            let mut encrypted_chars = encrypted_text.chars();

            while let Some(c) = encrypted_chars.next() {
                if let Some(s) = salt.chars().next() {
                    if let Some(original_char) = encrypted_chars.next() {
                        result.push(original_char);
                        result.push(s);
                        result.push(c);
                    }
                }
            }

            result
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("input username : ");
    let msg = io::get_inpt_user();
    print!("input salt : ");
    let salt = io::get_inpt_user();
    let urpass = mstr::generatestring(8);
    let hashed = utilities::password::bcryptor(&urpass);
    let cryp = utilities::password::cryptx(&urpass, &salt);
    let dec = utilities::password::decrypt(&urpass, &salt);
    println!(
        "username: {msg}, password: {hashed}, cryptx: {}, decryptx: {}",
        cryp, dec
    );
    /* let msg = io::get_inpt_user();
    //let msg = stakeinpt();
    let greet = mstr::multiply_name(&msg);
    println!("{greet}"); */
    //do_something();
    //let greeting: &str = parse("hi");
    //println!("{greeting}");
    //this(3, 5);
    //axray();
    // playing();

    Ok(())
}
