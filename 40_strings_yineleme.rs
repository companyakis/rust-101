//dize yineleme, string iteration

/*
örnekler
1- "mustafa" -> m, u, s, t, a, f, a
2- "rust dili" -> rust, dili
*/

//--------------------------------------------------------

//boşluğa göre bölme, strtr.split_whitespace()

fn main() {
 let cumle = "rust dili öğreniyoruz";
 for sozcuk in cumle.split_whitespace() {
   println!("{}", sozcuk);
 }
}

/*
rust
dili
öğreniyoruz
*/

//--------------------------------------------------------

//belirli bir işarete, karaktere göre bölme, örneğin virgül için str.split(",")

fn main() {
  let str = "Ali, Bilge, Ayşe, Kağan.".to_string();
  for ad in str.split(","){
      println!("{}", ad);
  }
}

//aşağıdaki kod sizce çalışır mı?

fn main() {
  let str = "Ali, Bilge, Ayşe, Kağan.".to_string();
  for ad in str.split("g"){
      println!("{}", ad);
  }
}

//--------------------------------------------------------

//dizenin her bir elemanını döndürme, str.chars()

fn main() {
  // define a String object
  let ad = "mustafa 1";
  for i in ad.chars() {
    println!("{}", i);
  }
}
