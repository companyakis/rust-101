/*
Vector nedir? Boyutu değiştirilebilen array gibi düşünebiliriz.
Array konusuna daha önce değinmiştik.
*/

fn main() {
   let bilgi = vec!["ad", "soyad", "yas", "doğum yeri"];
   println!("{:?}", bilgi);
}

//["ad", "soyad", "yas", "doğum yeri"]

//vector elemanına erişim v[index_value]

fn main() {
   let bilgi = vec!["ad", "soyad", "yas", "doğum yeri"];
   //ilk iki öğeyi yazdıralım
   println!("İlk eleman {:?}", bilgi[0]);
   println!("İkinci eleman {:?}", bilgi[1]);
}

//olmayan bir index numarası ile sorgulama yaparsak, hata alacağımızı unutmayalım!

fn main() {
   let bilgi = vec!["ad", "soyad", "yas", "doğum yeri"];
   println!("Olmayan eleman örneği {:?}", bilgi[10]);
}

//thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 10',

//yazdığımız programın çökmesini match kullanımı ile engelleyebiliriz!

fn main() {
    let bilgi = vec!["ad", "soyad", "yas", "doğum yeri"];

    match bilgi.get(10) {
      Some(eleman) => println!("{}", eleman),
      None => println!("Bu dizine ait veri bulunamadı!"),
    }
}
