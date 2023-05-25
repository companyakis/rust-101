//faktoriyel hesaplama uygulaması

//örneğin 5! = 5 * 4 * 3 * 2 * 1
//örneğin 100! = 100 * 99 * .... * 1

//Sıfırdan küçük sayılar için sonuç 0
//Sıfır faktoriyel için sonuç 1

//örnekte 7 için hesaplama yaptık, ama sayi değişkeninin değerini değiştirip, farklı denemeler yapabilirsiniz

fn main() {
    let mut faktoriyel = 1;
    let sayi = 7;
    
    if sayi < 0 {
        println!("0");
    } else if sayi == 0 {
        println!("1");
    } else {
        for s in 1..sayi+1 {
            faktoriyel = faktoriyel * sayi;
        }
        println!("{}", faktoriyel);
    }
}
// sonuç ->  823543
               
