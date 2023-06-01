/*
module rust dili için nedir? 

bir çuval düşünün ve farklı öğeleri içine dolduruyoruz.

module yapısı da struct, enum, function, vector, trait, array gibi yapıları içerebilir

böylece ne elde edeceğiz?
1- kod yapımız düzenlenmiş olacak
2- tekrar elde edilebilirlik sağlanmış olacak

örnek bir çatı:

mod module_name() {
  struct StructName {
  }
  fn function_name() {
  }
  enum EnumName {
  }
  ...
}

örnek kullanım => module_name::EnumName;

anahtar sözcükler: 
1- mod: yeni bir module tanımlama
2- pub: public bir module kullanımı (default olarak private olurlar) 
3- use: alanında, module dahil etme (import)

*/

//kolay bir örnek ekleyelim, module tanımlarken snake_case kullanalım

mod my_feelings {
    pub fn my_happiness() {
        println!("Rust öğreniyoruz diye çok mutluyum:)");
    }
    
    fn my_secret() {
        println!("Bu işlev private. Kapsam dışında hata alalım mı?")
    }
  
} 

fn main() {
    //my_happiness işlevi hatasız burada çalışacak
    my_feelings::my_happiness(); //çıktı -> Rust öğreniyoruz diye çok mutluyum:)
    
    //aşağıdaki kullanım hata verecek
    my_feelings::my_secret(); //çıktı -> error[E0603]: function `my_secret` is private
}
