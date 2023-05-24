//while döngüsü
//belirli bir koşul sağlanana kadar sürer, ama yineleme sayısı önceden bilinmemektedir

//bir örnek yapalım. biraz karmaşık gelebilir, ama farklı konuları hatırlamış olacağız

fn main() {
  let mut sayi = 3; 
  let mut hedef = false; 

  // !hedef -> true olacak
  while !hedef {
    sayi += 1; //sayı değeri 1 artıyor
    println!("Sayı değeri: {}", sayi);

    if sayi % 11 == 1 {
      hedef = true;
    }
    println!("Döngü şimdilik sürüyor😊");
  }
  println!("Döngü bitti ve sayının son değeri: {}", sayi);
}

/*
Sayı değeri: 4
Döngü şimdilik sürüyor😊
Sayı değeri: 5
Döngü şimdilik sürüyor😊
Sayı değeri: 6
Döngü şimdilik sürüyor😊
Sayı değeri: 7
Döngü şimdilik sürüyor😊
Sayı değeri: 8
Döngü şimdilik sürüyor😊
Sayı değeri: 9
Döngü şimdilik sürüyor😊
Sayı değeri: 10
Döngü şimdilik sürüyor😊
Sayı değeri: 11
Döngü şimdilik sürüyor😊
Sayı değeri: 12
Döngü şimdilik sürüyor😊
Döngü bitti ve sayının son değeri: 12
*/
