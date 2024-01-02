pub fn multiply_name(name: &str) -> String {
    let xor: String = "hi haxor ".to_owned();
    let nms: &str = name;
    let welcomer = xor + &nms + "\n";
    return welcomer.repeat(2).to_string();
}

pub fn generatestring(length: u32) -> String {
    use rand::Rng;
    let mut arr = Vec::new();
    for _ in 0..=length {
        let n: String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(1)
            .map(char::from)
            .collect();
        // let char = CHARSET.as_bytes()[n];
        arr.push(n);
    }
    arr.into_iter().collect::<String>()
    // std::str::from_utf8(&arr[..]).unwrap().to_string()
}
