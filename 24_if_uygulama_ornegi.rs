//bir sayının 47'ye bölümünden kalanın çift ya da tek olup olmasını rust ile kontrol edelim mi
//örnekte fayda aramayalım lütfen, yeni başlayanlar için uygulama ve pekiştirme amaçlıdır

fn main() {

    let sayi;
    
    sayi = u64::pow(5, 21); //5 sayısının 21. kuvveti, yani 5*5*5.....*5
    
    println!("{}", sayi);
    
    let kırkyedi_kalan = sayi % 47;
    
    println!("{}", kırkyedi_kalan);
    
    if kırkyedi_kalan % 2 == 0 {
        println!("{} sayısının 47'ye bölümünden kalan {} sayısı çifttir.", sayi, kırkyedi_kalan);
    } else {
        println!("{} sayısının 47'ye bölümünden kalan {} sayısı tektir.", sayi, kırkyedi_kalan);
    } 
}

/*
476837158203125
15
476837158203125 sayısının 47'ye bölümünden kalan 15 sayısı tektir.
*/
