//use anahtar sözcüğü
//içeri aktarma olarak düşünebiliriz
//böylece, içeri aktarım yapıldıktan sonra, bütün path'in yazılma zorunluluğu ortadan kalkar

pub mod kita {
    pub mod ulke {
        pub mod turkiye {
            pub fn turkiye_kita() {
                println!("Türkiye'nin çoğu Asya kıtasında.");
            }
            pub fn turkiye_izmir() {
                println!("İzmir en sevdiğim ildir!");
            }
        }
    }
}

//use ile path kısaltmış olacağız, zamanı etkin kullanma ve okunma kolaylığı
use kita::ulke::turkiye;

fn main() {
    //sadece turkiye ile iki işleve erişim sağlanmış oluyor
    
    turkiye::turkiye_kita();
    
    turkiye::turkiye_izmir();
}

/*
Türkiye'nin çoğu Asya kıtasında.
İzmir en sevdiğim ildir!
*/
