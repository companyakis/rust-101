// Rust temel kavramlar mustafa buyukdereli mayıs 2023

fn main() {
    
    // değişken oluşturma örneği
    
    let sayi= 5;
    
    // değişkeni yazıralım
    
    println!("Sayının değeri: {}", sayi);
    
    // değişkenin değerini değiştirmek istersek ne olacak?
    
    // hata alacağız, bu nedenle aşağıdaki kod yoruma alındı
    
    //sayi = 42;
    
    //println!("Sayının değeri: {}", sayi);
    
    //değişken değerini değiştirmek istersek, en başta mut demeliyiz
    // mut mutable
    //varsayılan olarak değişkenler en başta immutable olurlar, yani değerlerini değiştiremeyiz
    
    let mut ad = "Mustafa";
    
    println!("Adım: {}", ad);
    
    //adı değiştiriyoruz
    
    ad = "Bilge Kültigin";
    
    println!("Adım: {}", ad);
    
    //sabitler
    
    //örneğin pi sayısı, günde bulunan saat sayısı... bunlar değişmeyen değerler değil mi?
    
    //const ile tanımlıyoruz
    
    //her zaman büyük harfler kullanıyoruz
    
    //her zaman türlerini belirtiyoruz
    
    const GUN: i8= 24;
    
    println!("Bir günde bulunan saat sayısı: {}", GUN);
}
