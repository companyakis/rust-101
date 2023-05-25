//kullanıcıdan veri alan işlevler -user defined functions with parameters

//önceki bölümde 3 defa günaydın dedirttik, ama farklı kişlerin adlarıyla birlikte kullanmamız gerekirse?..
//örneğin "Günaydın Bilge!" ve "Günaydın Kağan!" deme gibi...

//------------------------------------
fn gunaydin_de(ad: &str) {
  println!("Günaydın {}!", ad);
}

fn main() {
  gunaydin_de("Bilge");
  gunaydin_de("Kağan");
}

/*
Günaydın Bilge!
Günaydın Kağan!
*/

//------------------------------------

//kullanıcı tarafından verilen iki sayıyı toplayan bir işlev yazalım

fn topla(a:i8, b:i8) {
  println!("{} + {} = {}", a, b, a + b);
}

fn main() {
  topla(4, 5); // 4 + 5 = 9
  topla(8, -12); // 8 + -12 = -4
}

/*
4 + 5 = 9
8 + -12 = -4
*/
