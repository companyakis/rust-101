//farklı türlerden veri tutabildiğimiz tuple konusunu işlemiştik
//struct konusu tuple'a benzer, ama veri türlerini belirtmek durumundayız

/*
şimdi, struct için bir örnek verelim
bir okuldaki öğrencilerin ad, soyad, öğrenci numarası bilgilerini struct ile takip edebiliriz
*/

//bir struct yapısı oluşturalım

struct Ogrenci {
   ad: String,
   soyad: String,
   ogrenci_no: u16,
}

fn main() {
   //bir öğrenci için Struct ile kayıt tutalım
   let ali = Ogrenci {
      ad: "Ali".to_string(),
      soyad: "Gök".to_string(),
      ogrenci_no: 443,
   };

   //ali'nin bilgilerini yazdıralım
   println!("Öğrencinin adı: {}", ali.ad);
   println!("Öğrencinin soyadı: {}", ali.soyad);
   println!("Öğrencinin numarası: {}", ali.ogrenci_no);
}

/*
Öğrencinin adı: Ali
Öğrencinin soyadı: Gök
Öğrencinin numarası: 443
*/
