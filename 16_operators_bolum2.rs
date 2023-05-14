//logical operators , mantıksal işlemciler
//true ve false ile işlem
// && ve
// || veya
// ! değil

fn main() {
  let x = true;
  let y = false;
  println!("İşlemci 1: {}, işlemci 2: {}", x , y);
  println!("Ve: {}", x && y);
  println!("Veya : {}", x || y);
  println!("Değil : {}", ! x);
  println!("Değil : {}", ! y);
} 

/*
Çıktılar aşağıda:

İşlemci 1: true, işlemci 2: false
Ve: false
Veya : true
Değil : false
Değil : true

*/
