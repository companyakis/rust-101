//dizede o parça var mı, substring arama
//str.contains("sub_string) -> true/false döner

fn main() {
  let ben = "Mustafa Büyükdereli".to_string();
  println!("{}", ben.contains("dereli")); //true döndü
}

//dizede değişiklik yapma
//str.replace(ilk, son)

fn main() {
  let ben = "mustafa büyükdereli";
  println!("{}", ben.replace("büyükdereli","rust 101")); // mustafa rust 101
}

//boşlukları kaldırma 
//string.trim()

fn main() {
   let cumle = "   Günler geçiyor ".to_string();
   println!("{}", cumle);
   let trim_cumle = cumle.trim(); 
   println!("{}", trim_cumle);
}
/*
   Günler geçiyor 
Günler geçiyor
*/
