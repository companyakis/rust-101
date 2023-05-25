//hafızada ne kadar yer (kaç byte) tahsis edildi? str.capacity()
//len() ve capacity() farklı  mıdır?!

//ilginç bir örnek ve çıktısı

fn main() {  
  let aga_1 = String::from("Rüştü Ağa");
  let aga_2 = String::from("Rustu Aga");

  println!("Rüştü Ağa Capacity: {}.", aga_1.capacity());
  println!("Rüştü Ağa Length: {}.", aga_1.len());

  println!("Rustu Aga Capacity: {}.", aga_2.capacity());
  println!("Rustu Aga Length: {}.", aga_2.len());
}

/*
Rüştü Ağa Capacity: 13.
Rüştü Ağa Length: 13.
Rustu Aga Capacity: 9.
Rustu Aga Length: 9.
*/
