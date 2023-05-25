//dizeye tek bir ekleme yapma, str.push()

fn main() {
  let mut dil = "Rus".to_string();
  //t harfini ekleyelim
  dil.push('t'); //çift tırnakta hata olacağını unutmayalım
  println!("{}", dil); //Rust
}

//-----------------------------------------------------

//dizeye çoklu ekleme yapma str.push_str()

fn main() {
  let mut ad = String::from("Bil");
  ad.push_str("ge Kağan ve kardeşi Kültigin");
  println!("{}", ad); //Bilge Kağan ve kardeşi Kültigin
}

//-----------------------------------------------------

//format macro ile birden fazla dize birleştirme
//sıralamayı kendimiz belirleyebiliriz, index 0'dan başlar mantığı burada da benzer

fn main(){
  let ad_1 = "aysel".to_string();
  let ad_2 = "güney".to_string();
  let soyad = "candan".to_string();
  
  let siralama = format!("{2} {0} {1}", ad_1, ad_2, soyad);
  
  println!("{}", siralama); //candan aysel güney
}
