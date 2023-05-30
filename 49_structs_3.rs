//struct yapısı için işlev tanımlayabiliriz

struct Dikdortgen {
    kisa_kenar: u32,
    uzun_kenar: u32,
}

//d harfi yerine başka bir ifade de kullanabilirdik

fn dıkdortgen_cevresi_hesapla(d: Dikdortgen) {
    println!("Dikdörtgenin çevresi: {}", 2 * (d.kisa_kenar + d.uzun_kenar));
}

fn main(){
let bir_dikdortgen = Dikdortgen {kisa_kenar: 23, uzun_kenar: 41};
    
dıkdortgen_cevresi_hesapla(bir_dikdortgen); //Dikdörtgenin çevresi: 128
}
