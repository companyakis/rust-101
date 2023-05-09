//tuple
//farklı türden verileri tutabiliriz
//örneğin adınız, soyadınız, doğum yılınız ve öğrenci numaranız birlikte gibi

let one_tuple = ("string burada", 'c', 2023); 

//tür belirterek tuple tanımlama

let my_tuple : (&str, char, i8) = ("Günaydın", 'a', 5);

//--------------------------------------------------

//tuple değerlerine erişne
//tuple_name.index_value

//tuple değerlerini parçalama, destructure olarak bilinir

let bir_kisi = ("Aygün", 1990, 33);
let (ad, dogum_yili, yas) = bir_kisi;

//----------------------------------------------------

//bütün bir örnek yapalım

fn main() {
    let my_tuple : (&str, char, i8) = ("Günaydın", 'a', 5);
    //ortadaki a harfini yazdıralım
    println!("Tuple ikinci eleman: {}", my_tuple.1);

    //destructure işlemi
    let bir_kisi = ("Aygün", 1990, 33);
    let (ad, dogum_yili, yas) = bir_kisi;
    println!("Ad: {}",ad);
    println!("Doğum yılı: {}", dogum_yili);
    println!("Yaş: {}", yas);

    //tuple yazdırma degug trait ile {:?}
    println!("Bütün tuple: {:?}", bir_kisi);

    //tuple mut ile değiştirilebilir tanımlanabilir
    let mut yillar = (2018, 2023);
    println!("Yılların ilk durumu: {:?}", yillar);
    //ilk elemanı değiştirelim
    yillar.0 = 1997;
    println!("Yılların son durumu: {:?}", yillar); 
}

/*
Bütün işlemler için çıktılar aşağıda:

Tuple ikinci eleman: a
Ad: Aygün
Doğum yılı: 1990
Yaş: 33
Bütün tuple: ("Aygün", 1990, 33)
Yılların ilk durumu: (2018, 2023)
Yılların son durumu: (1997, 2023)
*/
