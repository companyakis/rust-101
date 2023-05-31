/*
enum methods

impl EnumName {
  fn method_name(&self) {
  
  }
}

örnekle konuyu sürdürelim
*/

#![allow(dead_code)]
#[derive(Debug)]

enum TrafikIsiklari {
  Kirmizi,
  Yesil,
  Sari,
}

impl TrafikIsiklari {
  fn surucu_duracak(&self) -> bool {
    match self {
      TrafikIsiklari::Kirmizi => return true,
      _=> return false
    }
  }

}

fn main() {
  let isik_1 = TrafikIsiklari::Kirmizi;
  println!("Trafik lambası ışığı? {:?}", isik_1);
  println!("Sürücü duracak mı? {}", isik_1.surucu_duracak());

  let isik_2 = TrafikIsiklari::Sari;
  println!("Trafik lambası ışığı? {:?}", isik_2);
  println!("Sürücü duracak mı? {}", isik_2.surucu_duracak());
}
