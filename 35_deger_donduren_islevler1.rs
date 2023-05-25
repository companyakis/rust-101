/*
genel yazım örneği

fn my_func(param_1:dtype...,param_n:dtype) ->dtype {
  value //return value olarak kullanmak zorunlu değil
}
*/

//örnek yapalım: bir dikdörtgenin çevresini hesaplatalım

fn cevre_hesapla(kisa_kenar:f32, uzun_kenar:f32) -> f32 {
    (kisa_kenar + uzun_kenar) * 2.0
}

fn main() {
    //2 farklı dikdörtgen için hesaplama yapalım. veri türü olarak f32 belirttiğimize dikkat
    
    let mut a = 5.0;
    let mut b = 12.25;
    
    println!("Kısa kenarı {} cm. ve uzun kenarı {} cm olan dikdörtgenin çevresi {} cm. olur.", a, b, cevre_hesapla(a, b));

    a = 11.32;
    b = 23.0;
    
    println!("Kısa kenarı {} cm. ve uzun kenarı {} cm olan dikdörtgenin çevresi {} cm. olur.", a, b, cevre_hesapla(a, b));

}

/*
Kısa kenarı 5 cm. ve uzun kenarı 12.25 cm olan dikdörtgenin çevresi 34.5 cm. olur.
Kısa kenarı 11.32 cm. ve uzun kenarı 23 cm olan dikdörtgenin çevresi 68.64 cm. olur.
*/
