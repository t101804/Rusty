use std::io::Write;

pub fn get_inpt_user() -> String {
    let mut msg = std::string::String::new();
    let _ = std::io::stdout().flush();
    let res = std::io::stdin().read_line(&mut msg);
    if res.is_err() {
        println!("error : {:?}", res.err());
    }
    msg.trim().to_string()
}
