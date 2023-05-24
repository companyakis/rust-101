//enumerate işlemleri
//diğer dillerde de olan bir konu, değerlere sırayla numara verme gibi düşünebiliriz
//sıfırdan başlaması garipsenmesin, dizin mumarasının sıfırdan başlaması gibi...

fn main() {
  for (sira, deger) in (5..12).enumerate() {
    println!("Sıra: {} ve değer: {}", sira, deger);
  }
}

/*
Sıra: 0 ve değer: 5
Sıra: 1 ve değer: 6
Sıra: 2 ve değer: 7
Sıra: 3 ve değer: 8
Sıra: 4 ve değer: 9
Sıra: 5 ve değer: 10
Sıra: 6 ve değer: 11
*/
