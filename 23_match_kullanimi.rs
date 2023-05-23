//if else yapısında olduğu gibi, eşleştirme/seçim işlemlerini match ile yapabiliriz
//diğer bazı dillerde olan switch yapısına benzer

//kullanım 1: bir örnek yapacağız, ama (_ =>) ile belirtilmeyeni seçtiğimizi (default value diyebilir miyiz?) en başta belirtelim

 fn main() {
    let ad = "sena"; 
    //let ad = "mustafa";

    match ad {
        "mustafa" => println!("Adın Mustafa"),
        "aygün" => println!("Adın Aygün"),
        _ => println!("Farklı bir ad belirttiniz: {}", ad),
    };
}

//Farklı bir ad belirttiniz: sena

//-------------------------------------

//kullanım 2: sonucu en başta bir değişkene atayacağız ve daha sonra yazdırma yapalım

fn main(){
   let ad = "cemil";

   let aranan_ad = match ad {
      "mustafa" => "Adınız Mustafa!",
      "aygün" => "Adınız Aygün!",
      _ => "Çalışan listesinde olmayan bir ad belirttiniz..."
   };
   println!("Sonuç -> {}", aranan_ad);
}

//Sonuç -> Çalışan listesinde olmayan bir ad belirttiniz...
