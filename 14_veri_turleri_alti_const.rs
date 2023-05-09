//const 
//sabit değiştirilemeyen değişken tanımlama

//const var_name:type = value;

//genel kullanım BUYUK_HARFLERLE_TANIMLAMA , SCREAMING_SNAKE_CASE

const ADIM: &str = "MUSTAFA"; //global sabit değişken
fn main() {
    const PI_SAYISI: f32 = 3.14; //yerel local sabit değişken
    
    println!("Global sabit değişken: {}", ADIM);
    println!("Local sabit değişken: {}", PI_SAYISI);
}

/*
Çıktılar:

Global sabit değişken: MUSTAFA
Local sabit değişken: 3.14

*/
