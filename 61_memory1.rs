/*rust güçlü bir sistem dili ve hafıza yönetimi konusu çok önemli
daha önce de belirttim, rust dilinde stack ve heap kavramları önemli

=> stack kullanımı <=
Derleme zamanında verinin boyutu bilindiğinde...
LIFO kavramını unutmayalım: Son giren ilk çıkar, Last In First Out
Sabit bir boyutu olan tüm ilkel veri türleri için  
                            
=> heap kullanımı <=                            
büyük bir veri deposudur ve derleme zamanında boyutu bilinmeyen değerleri depolar      
Vectors ve string nesnelerinin boyutları sabit değil, heap'te depolanırlar  
  
===> sahiplik konusuna giriş yapalım, ownership <===
                         
hafıza yönetimi için önemli bir konu olan ownership için 3 konuya dikkat!
- her değerin bağlayıcı bir sahibi vardır
- yalnızca bir sahip olabilir
- kapsam dışında, sahiplik ortadan kalkar
  
basit örneklerle konu aşağıda açığa çıksın
*/

fn main () {
    let a = 1; //bu 1 değerinin sahibi a'dır
    let b = 1; //bu 1 değerinin sahibi b'dir

    //o zaman şunu diyebiliriz: aynı 1 değerine iki farklı değişken sahip olamaz
    
    {
        let c = 9; //c bu kapsamda 3 değerinin sahibidir
    }
    //üçüncü kural devreye girer: kapsam dışında sahiplik olamaz
    println!("{}", c); //c burada sahip değil, hata alacağız
}
