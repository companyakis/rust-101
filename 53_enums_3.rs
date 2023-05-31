//enum ve match birlikte bir örnek yapalım

#[warn(dead_code)]

enum Diller {
   Rust,
   Python,
   Scala,
   JavaScript,
   Solidity,
}

fn ogrenme_sureci(dil: Diller) {
   match dil {
      Diller::Rust => {
         println!("Rust öğreniyoruz!");
      },
      Diller::Python => {
         println!("Python öğreniyoruz!");
      },
      Diller::Scala => {
         println!("Scala öğreniyoruz!");
      },
       Diller::JavaScript => {
         println!("JavaScript öğreniyoruz!");
      },
      Diller::Solidity => {
         println!("Solidity öğreniyoruz!");
      }
   }
}

fn main() {
    let rust = Diller::Rust;
    let python = Diller::Python;
    let scala = Diller::Scala;
    let javascript = Diller::JavaScript;
    let solidity = Diller::Solidity;

    ogrenme_sureci(rust);
    ogrenme_sureci(python);
    ogrenme_sureci(scala);
    ogrenme_sureci(javascript);
    ogrenme_sureci(solidity);
}

/*
Rust öğreniyoruz!
Python öğreniyoruz!
Scala öğreniyoruz!
JavaScript öğreniyoruz!
Solidity öğreniyoruz!
*/
