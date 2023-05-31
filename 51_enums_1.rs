/*
enum, değişkenlerden oluşan özel bir veri türüdür.
tüm olası değerleri numaralandırma ve listeden değerlerden birini seçmek...
trafik ışığı örneğini verebiliriz
CamelCase kullanimini unutmayalim

örnekle daha iyi kavrayacağız
*/

#[derive(Debug)]

enum TrafikIsiklari {
   Kirmizi,
   Sari,
   Yesil,
}

fn main() {
   let kirmizi = TrafikIsiklari::Kirmizi;
   let sari = TrafikIsiklari::Sari;
   let yesil = TrafikIsiklari::Yesil;

   println!("{:?}", kirmizi);
   println!("{:?}", sari);
   println!("{:?}", yesil);
}
