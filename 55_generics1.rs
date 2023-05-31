/*
veri türlerinin genelleştirilmesi ve veri türünün çalışma zamanında (run time) belirlenmesi olarak kısaca tanımlayabilir miyiz

sahada çarpışan askerlerin rütbeleri olmaz; çünkü düşmanın kimin komutan olduğunu ve kimlerin hangi rütbelere sahip olduklarını kolayca anlamaması amaçlanır

verdiğim örnek %100 uyuşmamış olabilir, ama uygulama ile kavrayacağız

structs, functions, trait, enum, vector, collection... hepsine de uygulanabilir
*/

//örnek yapacağız. string ile integer kullanacağız

//<T> anahtar sözcüğü generic tanımlama için önemli

use std::fmt::Display; //unutmayalım

fn birlestir<T:Display>(oge_1:T, oge_2:T) {
    let birlesen_ogeler = format!("{}{}", oge_1, oge_2); //türleri belirttik mi
    println!("{}", birlesen_ogeler);
}

fn main() {
    println!("Burada String kullanıyoruz!");
    birlestir("Rust dili", " 101");
    println!("Burada Int kullanıyoruz!");
    birlestir(20 as u8, 23 as u8);
}
  
/*
Burada String kullanıyoruz!
Rust dili 101
Burada Int kullanıyoruz!
2023
*/
