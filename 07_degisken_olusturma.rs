//bir değişken oluşturma örneği
//bu değişken immutable, değiştirilemez

fn main() {
    let bir_degisken = 1923;
    println!("Kuruluş tarihi: {}", bir_degisken);
}

//----------------->yeni bölüm aşağıda-------------

//mutable değişken, mut ile tanımlıyoruz

fn main() {
    let mut ulke = "Türkiye";
    println!("Ülke: {}", ulke);
    ulke = "Selçuklu";
    println!("Ülke: {}", ulke);
}


//----------------->yeni bölüm aşağıda-------------

//aynı anda birden fazla değişken oluşturabiliriz

fn main() {
   let (lang_1, lang_2, lang_3, lang_4, lang_5) = ("Rust", "Python", "JavaScript", "Scala", "Dart");
   println!("Mustafa Buyukdereli'nin kullandığı diller: {}, {}, {}, {} ve {}.", lang_1, lang_2, lang_3, lang_4, lang_5);
}


//----------------->yeni bölüm aşağıda-------------

//scope
//hata alacağız

fn main() {
  let dis_degisken = 1923; //global variable
  { // start of code block
        let ic_degisken = 0; //local variable desek hatalı olmaz gibi?
        println!("Dış değiken: {}", dis_degisken);
        println!("Yerel değişken: {}", ic_degisken);
  } 
  println!("Yerel değişken: {}", ic_degisken); // yerel değişken için scope sorunu, hata alacağız
}
