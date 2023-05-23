//if kullanımı

fn main() {
      let ad = "Mustafa";

      if ad == "Mustafa" {
            println!("Ad: {}", ad);
      }
}

//-----------------------------------------------------------------------

//if else örneği
//ikili durumlarda kullanabiliriz

fn main() {
  let yas = 17;

  if yas > 18 {
    println!("Erişkinsiniz.");
  } else {
    println!("Erişkin değilsiniz ve yaşınız: {}.", yas);
  }
}

//çıktı -> Erişkin değilsiniz ve yaşınız: 17.

//---------------------------------------------------------------------------

//if, else if ..., else
//çoklu koşulların kontrolü sağlanır
//else if kaç tane kullanılacak, gereksinime bağlıdır

fn main() {
   let il = "Ankara";

   if il == "İzmir" {
      println!("İzmir Ege'nin incisidir.");
   } else if il == "Ankara" {
      println!("Başkent Ankara'yı seviyor olmalısınız.");
   } else if il == "İstanbul" {
      println!("Fatih'te mi yaşıyorsunuz?");
   } else {
      println!("Evet, farklı bir il: {}.", il);
   }
}

//çıktı -> Başkent Ankara'yı seviyor olmalısınız.

//---------------------------------------------------------------

//iç içe if kullanımı, nested if yapısı

fn main() {
    //define a variable 
    let ulke = "Tayland";
    let il = "Ankara";
    // outer if statement
    if ulke == "Türkiye" {  // inner if statement
        if il == "Ankara"{
              println!("Türkiye'nin başkentindesiniz.");
        }
    }
    else {
      println!("Durumunuz çok karışık:)");
    } 
}

//çıktı -> Durumunuz çok karışık:)

//-------------------------------------------------------------

//if kısa kullanımı, ternary kullanımına benzemiyor mu

fn main() {
    let yas = 24;  
    let sonuc = if yas < 18 {"Yetişkin değilsiniz!"} else {"Yaşınız 18'den büyük ve yetişkinsiniz!"};
    println!("{}", sonuc);
}

//çıktı -> Yaşınız 18'den büyük ve yetişkinsiniz!                            
                     
fn main() {
    let x = "a";

    let y: bool = if x == "g" { true } else { false };

    println!("x:{}", x);
    println!("y:{}", y);
}
      
/*
x:a
y:false
*/
      
      
      
      
      
      
  
