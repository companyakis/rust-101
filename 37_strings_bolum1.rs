/*
String konusu Rust'ın zorlayıcı konularından biridir.
Heap ve Stack konularını anladıysanız, burada zorlanmayacaksınız.
YouTube'da Heap ve Stack Memory farkını anlatan güzel videolar vardır
Kavramları Türkçe olarak kullanmaya çalışsam bile, örneğin String Literal gibi bir kavramı "Dize Değişmezi" olarak bilmemiz kafa karıştırıcı olabilir.
Çünkü Rust diliyle ilgili Türkçe eğitim, kaynak vd bulmak zor.
*/

//String Türleri

//string literal -> primitive type, değişmez-immutable, stack'te tutulur, string slice kavramı da ussumuzda dursun

fn main() {
  let dil:&str = "Rust dili";
  println!("Değişmez dize örneği: {}", dil);
  //len işlevi ile değişmez bir dizenin uzunluğu bulma
  println!("Değişmez dize uzunluğu: {}", dil.len()); //8 çıkmaz; çünkü iki sözcük arası boşluk var!
}

//string object -> dize nesnesi, heap'te tutulur, uzunluğu değiştirilebilir

//boş bir string object oluşturalım
let ad = String::new(); //boş
let ad_buyut = ad.to_string();

//bir string object örneği
let benim_adim = String::from("Mustafa");

//string literal değişmez demiştik, string object dönüşümü yapalım
let sayi = "Sekiz"; //literal
let sayi2 = sayi.to_string(); //object
