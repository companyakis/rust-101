//loop ile sonsuz döngü!
//örnekte sayı değerini her seferinde 3 arttıralım

fn main() {
  let mut sayi = 13;

  println!("Sayının ilk değeri: {}", sayi);

  //loop ile sürekli 13 artan bir döngü

  loop {
    sayi += 13; // sayi = sayi + 13

    println!("Sayının yeni değeri: {}", sayi);
  }
}

/*
Sayının ilk değeri: 13
Sayının yeni değeri: 26
Sayının yeni değeri: 39
Sayının yeni değeri: 52
Sayının yeni değeri: 65
Sayının yeni değeri: 78
Sayının yeni değeri: 91
Sayının yeni değeri: 104
Sayının yeni değeri: 117
Sayının yeni değeri: 130
Sayının yeni değeri: 143
Sayının yeni değeri: 156
Sayının yeni değeri: 169
Sayının yeni değeri: 182
Sayının yeni değeri: 195
Sayının yeni değeri: 208
Sayının yeni değeri: 221
Sayının yeni değeri: 234
Sayının yeni değeri: 247
Sayının yeni değeri: 260
Sayının yeni değeri: 273
Sayının yeni değeri: 286
...
...
*/
