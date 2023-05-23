/* 
for döngüsü:

kullanım şekli:
for var in range {
  ...
}

range: döngü aralığı. örneğin 0 ile 10 arasındaki sayıları 0..10 ile belirtiyoruz
0 sayısı dahil, ama 10 sayısı dahil olmamış oluyor.

aşağıya hemen bir örnek ekleyelim
*/

fn main() {
    //0 ile 6 arasındaki sayıları ekrana yazdıralım
    for sayi in 0..7 {
      println!("Sayı_{}:{}", sayi+1, sayi); //index 0'dan başladığı için (sayi+1) dedik
    }
}

/*
Sayı_1:0
Sayı_2:1
Sayı_3:2
Sayı_4:3
Sayı_5:4
Sayı_6:5
Sayı_7:6
*/
