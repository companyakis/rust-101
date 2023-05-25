//birden fazla değer döndüren işlevler

/*
fn my_func(param_1:dtype,..., param_n:dtype) -> (dtype_1,..., dtype_n) {
  (value_1,..., value_n)
}

//örnek: bir dikdörtgenin çevresini ve alanını hesaplayan bir işlev yazalım

fn main() {

    let uzunluk = 14;
    let genislik = 23;

    let (alan, cevre) = dikdötrgen_alan_cevre(uzunluk, genislik);
    
    println!("Alan: {} ve Çevre: {}", alan, cevre);
}

fn dikdötrgen_alan_cevre(kenar_1: u64, kenar_2: u64) -> (u64, u64) {
    
    let alan = kenar_1 * kenar_2;
    let cevre = 2 * (kenar_1 + kenar_2);
    //alan ve çevre
    (alan, cevre)
}
