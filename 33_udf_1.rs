//kullanıcı tanımlı işlevler - user defined functions

//kodları yalnızca bir defa kullanmayıp, farklı durumlarda da kullanmak istiyorsak, bir işlev yazarak bunu yapabiliriz
//örneğin 2 sayıyı toplayan basit bir işlev düşünelim. bu işlevi iki sayıyı toplama işlemlerinin hepsinde kullanabiliriz

//örnekte günaydın diyen bir işlev yazıyoruz ve bunu art arda 3 defa kullanıyoruz/çağırıyoruz

fn gunaydin_de() {
  print!("Günaydın! ");
}

fn main() {
  gunaydin_de();
  gunaydin_de();
  gunaydin_de();
}

// Günaydın! Günaydın! Günaydın! 
