//single placeholder

fn main() {
    println!("Yıl: {}", 2023);
}

// çıktı :  Yıl: 2023

//-------->yeni konu aşağıda--------------

//multiple placeholders

fn main() {
    println!("{} ile {} evreninde önemli olanaklar var mı?", "Rust", "Web3");
}

// çıktı  :  Rust ile Web3 evreninde önemli olanaklar var mı?

//-------->yeni konu aşağıda--------------

//positional arguments

fn main() {
    println!("{1}, {0} kıtasında yer alan bir ülke mi?", "Asya", "Bangladeş");
}

// çıktı :  Bangladeş, Asya kıtasında yer alan bir ülke mi?

//-------->yeni konu aşağıda--------------

//named arguments

fn main() {
    println!("{ulke} {yil} yılında kuruldu.", ulke="Türkiye", yil=1923);
}

// çıktı  :  Türkiye 1923 yılında kuruldu.

//-------->yeni konu aşağıda--------------

//temel matematik

fn main() {
    println!("{} + {} = {}",20, 10, 20 + 10);

    println!("{} * {} = {}", 30, 12, 30 * 12);
}

// çıktı  :  20 + 10 = 30
// çıktı  :  30 * 12 = 360


//-------->yeni konu aşağıda--------------

// debug trait ile tek placeholder kullanımı

fn main() {
    println!("{:?}", ("Rust temelleri", 101));

    println!("{:?}", ("Rust", "Javascript", "Python", "Scala", "Solidity"));
}

// çıktı  :  ("Rust temelleri", 101)
// çıktı  :  ("Rust", "Javascript", "Python", "Scala", "Solidity")
