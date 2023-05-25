//döngülerde "continue" kullanımı

//örnekle açıklayalım mı? bir odaya girdiniz ve 5 kişi var ve bunlardan bir kişiyi sevmiyorsunuz. sırayla herkese "günaydın" diyorsunuz
//sevmediğiniz kişiye günaydın demeyip, sıradaki kişiye günaydın demeye devam ediyorsunuz
//break kullansaydık, günaydın demeye devam etmezdik, ama continue ile sadece sevmediğimiz kişiyi es geçip, diğer kişilere günaydın demeye devam ediyoruz

fn main() {
  for sayi in 0..8 {
    if sayi == 6 {
      continue;
    }
    println!("Günaydın sayı: {}", sayi);
  }
}

/*
aşağıda görüldüğü üzere, sayı 6'yı continue ile geçtik!

Günaydın sayı: 0
Günaydın sayı: 1
Günaydın sayı: 2
Günaydın sayı: 3
Günaydın sayı: 4
Günaydın sayı: 5
Günaydın sayı: 7
*/
