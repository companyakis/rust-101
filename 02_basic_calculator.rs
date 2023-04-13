//basit calculator uygulaması
//mustafa buyukdereli nisan 2023

use std::io;


fn main() {
    //kullanıcıdan 2 sayı girmesini bekleyeceğiz
    
    let mut sayi_1 = String::new();
    let mut sayi_2 = String::new();
    
    println!("İlk sayı: ");
    io::stdin()
        .read_line(&mut sayi_1)
        .expect("İlk sayı girilmedi!");
    
    let sayi_1:f32 = sayi_1.trim().parse().expect("İlk sayı elde edilemedi!");
    
    println!("İkinci sayı: ");
    io::stdin()
        .read_line(&mut sayi_2)
        .expect("İkinci sayı girilmedi!");
    
    let sayi_2:f32 = sayi_2.trim().parse().expect("İkinci sayı elde edilemedi!");
    
    println!("Toplama işlemi için 1'e basınız.");
    println!("Çıkarma işlemi için 2'ye basınız.");
    println!("Çarpma işlemi için 3'e basınız.");
    println!("Bölme işlemi için 4'e basınız.");
    
    let mut islem = String::new();
    
    io::stdin()
        .read_line(&mut islem)
        .expect("İşlem türü belirtilmedi!");
        
    let islem:i32 = islem.trim().parse().expect("İşlem türü elde edilemedi!");
    
    //kullanıcı seçimine göre, işlem sonuçlarını dönderelim
    
    match islem {
        1 => toplama(&sayi_1, &sayi_2),
        2 => cikarma(&sayi_1, &sayi_2),
        3 => carpma(&sayi_1, &sayi_2),
        4 => bolme(&sayi_1, &sayi_2),
        _ => println!("Hatalı işlem türü belirtildi!"),
    }
   
}

// işlemleri gerçekleştirecek işlevleri tanımlayalım

fn toplama(veri1: &f32, veri2: &f32) {
    println!("Toplama işleminin sonucu: {}", veri1 + veri2)
}

fn cikarma(veri1: &f32, veri2: &f32) {
    println!("Çıkarma işleminin sonucu: {}", veri1 - veri2)
}

fn carpma(veri1: &f32, veri2: &f32) {
    println!("Çarpma işleminin sonucu: {}", veri1 * veri2)
}

fn bolme(veri1: &f32, veri2: &f32) {
    println!("Bölme işleminin sonucu: {}", veri1 / veri2)
}
