/* 
Type Casting
Özetle, şunları desem: Veri türleri arasında dönüşüm yapma...
"as" + dönüşüm yapılacak veri türü
*/

//aşağıdaki kodu çalıştırırsak hata alırız. bu nedenle type casting önemli

fn main() {
    let a = 15;
    let bolum = a / 2.5;
    println!("Bölme işlemi sonucu: {}", bolum);
}

// aldığımız hatayı aşağıya ekliyorum
// error[E0277]: cannot divide `{integer}` by `{float}`
// tam sayıyı ondalıklı sayıya bölemezsin diyor. matematikte bu var, ama Rust bu:)

//type casting ile hatayı düzeltelim: "a as f32" olarak ekleme yaptım! 

fn main() {
    let a = 15;
    let bolum = a as f32 / 2.5;
    println!("Bölme işlemi sonucu: {}", bolum);
}
   
//çıktı -> Bölme işlemi sonucu: 6

/*
temel kurallar: 
1- integer değerler float ya da float değerler integer olarak dönüştürülebilir
2- diğer dillerde olduğu üzere, örneğin 2 sayısını "2" olarak String de yapabiliriz
3- "Günaydın" ya da 'a' elbette type casting için uygun olmaz. ("Günaydın" as f64 saçma değil mi?)
4- String char ve char da string olarak type casting olmuyor.
5- ... temel olarak belirttim.
*/

//hata verecek bir örnek aşağıda

fn main() {
    let bir_harf: char = 'a' ; // char'dan string'e dönüş hatalı
    let donusturme = bir_harf as &str ; 
    println!("bir_harf: {}", bir_harf);
    println!("donusturme: {}", donusturme);
}

//hata -> error[E0605]: non-primitive cast: `char` as `&str`
