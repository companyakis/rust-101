//module dışında tanımlanan bir işlevi module içinde kullanım örneği yapalım

fn main() {

    println!("Bütün duygularım açığa döküldü:)");
    
    duygular::benim_duygularim_1();
}

//main işlevi dışında tanımladığımız bir işlev
fn benim_duygularim_2() {
        println!("Seni seviyorum 2");
}


mod duygular {
    pub fn benim_duygularim_1() {
        println!("Seni seviyorum 1");
        super::benim_duygularim_2();
    }
}

/*
Bütün duygularım açığa döküldü:)
Seni seviyorum 1
Seni seviyorum 2
*/
