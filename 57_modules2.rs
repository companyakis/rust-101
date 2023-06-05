//pub ve privacy konusuna devam edelim ve örneklerle farklı uygulamaları gösterelim

mod duygular {
    //default olarak private bir işlev
    fn gizli_duygular() {
        println!("Duygularimi saklamak istiyorum");
    }
    
    //public yapılan bir işlev, pub
    pub fn acik_duygular() {
        println!("Duygularimi duzgun bir sekilde evrene iletmeliyim!");
        println!("Aşağıya dikkat! Gizli duygu kalmasin:)");
        //ilk tanımladığımız işlevi burada kullanalım
        self::gizli_duygular();
    }
}

fn main() {
    println!("Duygularin hepsi aşağıda!");
    //çıkan sonuca dikkat
    duygular::acik_duygular();
}

/*
Duygularin hepsi aşağıda!
Duygularimi duzgun bir sekilde evrene iletmeliyim!
Aşağıya dikkat! Gizli duygu kalmasin:)
Duygularimi saklamak istiyorum
*/
