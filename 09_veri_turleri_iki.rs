//integer veri türleri

//i8, i16, i32, i64 -> signed integer, positive ya da negative değer alabilirler

//u8, u16, u32, u64 -> unsigned integer, yalnızca positive değer alabilirler

fn main() {
    let a:i8 = 24;
    let b:i64 = -46578245;

    let c:u16 = 42154;

    println!("a:{}, b:{} ve c:{}", a, b, c);
}

//a:24, b:-46578245 ve c:42154

//implicit tanımlama da olanaklıdır

fn main() {
    let dogum_yilin = 1990;

    println!("Doğum yılın: {}", dogum_yilin);   
}

// --------------------------------------

//float veri , ondalıklı veri diyebiliriz

//f32 ve f64

fn main() {
    let alan:f32 = 412.45;

    let pi = 3.14;

    println!("Dairenin alanı {} m^2 ve pi sayısı {} ise dairenin yarıçapı kaç metredir?", alan, pi);
}
