//dize dilimleme, dizeden belirli bölümleri alma, string slicing

//&string[baslangic..bitis]

//& dilim değişkeninin dizeyi ödünç aldığını gösterir

fn main() {
   let abc = String::from("ABCDEFGHIJKLM");
   //ilk beş harfi alalım, index 0'dan başlar mantığı geçerli
   let dilim = &abc[0..5];
   println!("{}", dilim); // ABCDE
}
