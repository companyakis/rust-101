/*
Structs içinde methods oluşturma

genel kullanım görüntüsü aşağıda belirtilmiştir

impl StructName {
  fn method_name(&self) {
  }
}

let instance_struct = StructName {item_1.value, ..., item_n.value}

method çağırma -> instance_struct.method_name();

neden struct methods? -> düzenli bir kullanım, impl içinde bir düzen...

*/

//ayrıntılı bir örnek ile daha iyi kavrayacağız

//struct ve struct method oluşturalım

struct Calisanlar {
    ad: String,
    soyad: String,
    ise_giris_yili: u16,
    birim: String,
    evli: bool,
}

impl Calisanlar {
    fn calisan_bilgilerini_yazdir(&self) -> String {
        if self.evli == true {
            format!("Çalışan adı ve soyadı: {} {}, işe giriş yılı: {}, birimi: {} ve evli.", self.ad, self.soyad, self.ise_giris_yili, self.birim)
        } else {
            format!("Çalışan adı ve soyadı: {} {}, işe giriş yılı: {}, birimi: {} ve bekar.", self.ad, self.soyad, self.ise_giris_yili, self.birim)
        }
       
    }
}

fn main() {
    //struct instance oluşturalım
    let finans_ali = Calisanlar {
        ad: "Ali".to_string(),
        soyad: "Güneyli".to_string(),
        ise_giris_yili: 2010,
        birim: "Finans".to_string(),
        evli: false,
    };

    //struct method ile bilgileri elde edelim

    println!("Ali Güneyli kişisel bilgileri => {:?}", finans_ali.calisan_bilgilerini_yazdir());
}

//Ali Güneyli kişisel bilgileri => "Çalışan adı ve soyadı: Ali Güneyli, işe giriş yılı: 2010, birimi: Finans ve bekar"
