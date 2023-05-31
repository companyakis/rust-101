/*
trait anlatması zor bir konu, ama "herkesi aynı kefeye koymak" deyimi ussumuzda bulunsun.

örneğin farklı şekillerin alanlarını hesaplayalım:
1- daire
2- kare
3- dikdörtgen
4- beşgen
...

bütün bu farklı şekillerin alanlarını hesaplarken, aynı ada sahip bir işlev kullansak?

örnekle yapalım ve anlamaya çalışalım

genel yapıyı önce aşağıya ekleyeyim

impl TraitName for StructName {
  fn method(&self) {
  }
}
*/

//------------------------------------------------------- örnek aşağıda-----------

struct Daire {
    yaricap: f64,
}

struct Kare {
    kare_kenar: f64,
}

struct Dikdortgen {
    kisa_kenar: f64,
    uzun_kenar: f64,
}

//trait tanımlayalım

trait Alan {
    fn alan_hesapla(&self) -> f64;
}

//trait uygulama, aynı alan_hesapla() işlevi kullanacağız!

impl Alan for Daire {
    fn alan_hesapla(&self) -> f64 {
        3.14 * self.yaricap * self.yaricap
    }
}

impl Alan for Kare {
    fn alan_hesapla(&self) -> f64 {
        self.kare_kenar * self.kare_kenar
    }
}

impl Alan for Dikdortgen {
    fn alan_hesapla(&self) -> f64 {
        self.kisa_kenar * self.uzun_kenar
    }
}

fn main() {
    //daire, kare ve dikdötgen için, Struct yapılarını kulllanarak, örnek oluşturalım
    let d = Daire { yaricap: 5.0 }; //daire için
    let k = Kare { kare_kenar: 15.0 }; //kare için
    let dd = Dikdortgen { kisa_kenar: 10.0, uzun_kenar: 20.0 }; //dikdörtgen için
    
    //alanları yazdıralım
    println!("Dairenin alanı: {}", d.alan_hesapla());
    println!("Karenin alanı: {}", k.alan_hesapla());
    println!("Dikdörtgenin alanı: {}", dd.alan_hesapla());
}

/*
Dairenin alanı: 78.5
Karenin alanı: 225
Dikdörtgenin alanı: 200
*/
