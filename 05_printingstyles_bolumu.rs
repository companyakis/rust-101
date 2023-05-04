
// print!()

// tek satırda baskı, yeni satır yok

fn main() {
    print!("Rust Programming");
    print!(" Web3");
}

// çıktı  :  Rust Programming Web3

//-------->yeni konu aşağıda--------------

// println!()

// yeni satırda devam ettirir

fn main() {
    println!("Rust Programming");
    println!("Web3");
}

// çıktı :  Rust Programming
// çıktı :  Web3

//-------->yeni konu aşağıda--------------

// eprint!()

// hatalı gibi uyarı ve tek satırda çıktı

fn main() {
    eprint!("Rust Programming");
    eprint!(" Course");
}

// çıktı :  Rust Programming Course

//-------->yeni konu aşağıda--------------

// eprintln!()

// hatalı gibi uyarı ve yeni satırda çıktı

fn main() {
    eprintln!("Rust Programming");
    eprintln!("Web3");
}

// çıktı :  Rust Programming
// çıktı :  Web3
