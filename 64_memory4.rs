//ownership ve function örneği yapalım

//ilk örnek string örneği olsun

fn main () {
    let cumle = "Günaydın diyelim!".to_string();
    
    cumle_turevi(cumle);
    
    println!("{}", cumle); //hata aldırır, ama yukarıda cumle_turevi(cumle.clone()) kullanırsak, hata yok
}

fn cumle_turevi(c: String) {
    println!("{}", c);
}

//ikinci örnek int örneği olsun

fn main () {
    let sayi = 101;
    
    sayi_sahiplik(101); //101
    
    println!("sayı: {}", sayi);  //101
}

fn sayi_sahiplik (s: i32) {
    println!("sayı: {}", s);
}
