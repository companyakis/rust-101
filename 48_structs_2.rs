//struct verisinde güncelleme yapmak istersek, tanımda mut olmalıdır!

//struct oluşturalım 

struct Dil {
   ad: String,
   numara: u8,
}

fn main() {
   let mut egitim_dili = Dil {ad: "JavaScript".to_string(), numara: 101};

   println!("Eğitim dili: {} ve kapsam numarası: {}", egitim_dili.ad, egitim_dili.numara);

   //Eğitim dilini Rust olarak güncelleyelim

   egitim_dili.ad = String::from("Rust");

   println!("Eğitim dili: {} ve kapsam numarası: {}", egitim_dili.ad, egitim_dili.numara);
   
}

/*
Eğitim dili: JavaScript ve kapsam numarası: 101
Eğitim dili: Rust ve kapsam numarası: 101
*/
