//arrays
//çoklu veri tutma
//aynı tür birden fazla veriyi tutma, örneğin çocuklarınızın doğum yıllarını bir liste içerisine alma gibi
//varsayılan olarak immutable
//başlangıç index 0. Ki bildiğim kadarıyla, Lua dili dışında, diğer diller de böyle

//array tanımlama, tür ve boyut en başta belirtilmelidir

//let my_array:[type; size] = [val1, val2, ..., valn];

fn main() {
   //üç elemanlı bir array tanımlayalım
   let yillar:[i32;3] = [1990, 2013, 2016];
   //yazdırırken {:?} ekleyelim, debug trait deniyor
   println!("{:?}", yillar);

   //bütün değerleri sıfır olan 5 elemanlı bir array tanımlayalım
   let bes_sifir = [0; 5];
   println!("{:?}", bes_sifir);

   //char değerlerle dolu bir array
   let adim:[char;7] = ['m', 'u', 's', 't', 'a', 'f', 'a'];
   println!("{:?}", adim);

   //string array oluşturalım
   let adlar:[&str;3] = ["Bilge", "Kültigin", "Göktürk"];
   println!("{:?}", adlar);

   //array elemanlarına index ile erişim
   //yukarıda tanımladığımız yıllar array'inin son elemanını yazdıralım
   //index değerleri -> ilk eleman 0, diğeri 1 ve sonuncu 2
   println!("Yılların son elemanı: {:?}", yillar[2]);
}

//-----------------------------------

//mut anahtar sözcüğü ile bir array değerleri değiştirilebilir

fn main() {
    //değiştirilebilir 4 elemanlı sayısal bir array
    let mut sayilar:[i8;4] = [3, 4, 5, 6];
    println!("Sayıların üçüncü elemanı: {:?}", sayilar[2]);
    sayilar[2] = 100;
    println!("Sayıların üçüncü elemanı: {:?}", sayilar[2]);
    println!("Sayıların en son durumu: {:?}", sayilar);
}
/*
işlemlerin çıktılarını kontrol edebilirsiniz:
Sayıların üçüncü elemanı: 5
Sayıların üçüncü elemanı: 100
Sayıların en son durumu: [3, 4, 100, 6]
*/

//---------------------------------------------------------------------

//array uzunluğunu yazdırma, gömülü (built-in) işlev len() ile

fn main() {
    //beş elemanlı bir array oluşturalım
    let arr:[i32;5] = [1, 2, 3, 4, 2023]; 
    //array uzunluğu 5 olarak yazılacak
    println!("Array uzunluğu: {}", arr.len());
}

//-------------------------------------------------------------------

//array dilimleme, slicing
//bir array için dilediğimiz bölümleri alma

//let slice_array:&[i8] = &my_array[0..3] gibi
//0, 1 ve 2 index değerli verileri alıyoruz, son index dahil değil, exclusive! 

fn main() {
    //5 elemanlı bir array
    let yillar:[i16;5] = [1947, 1966, 1982, 1993, 2023];
    println!("Yılların bütünü: {:?}", yillar);

    //ilk dört elemanı alalım
    let dilim_yillar:&[i16] = &yillar[0..4];
    println!("İlk dört yıl: {:?}", dilim_yillar);
}

//yukarı için çıktılar
//Yılların bütünü: [1947, 1966, 1982, 1993, 2023]
//İlk dört yıl: [1947, 1966, 1982, 1993]
