//boolean veri türü -> true ya da false değeri alır, mantıksal sınama...

//explicit definition, veri türünü belirtme örneği

fn main() {
    //rust dilini sevdiniz mi
   let rust_duygu:bool = true;
   println!("Rust dilini sevdiniz: {}", rust_duygu);
}

//implicit definition, veri türünü açıkça belirtmiyoruz

fn main() {
   let dogru = true;
   let yanlis = false;

   println!("Doğru: {} ve yanlış: {}", dogru, yanlis);
}

//işlem sınama sonucu örneği

fn main() {
    let a = 5 > 4;
    let b = 3==5;
    let c = 12 >24;
    let d = 5!=10;

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
}
