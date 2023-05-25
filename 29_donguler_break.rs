// Koşullu durumlarda, belirli bir koşul sağlandığında, döngünün sonlanması için "break" anahtar sözcüğünü kullanabiliriz

//for, while ve loop için 3 örnek aşağıda olacak
//i = i + 1 ya da i += 1 yazmanın aynı sonucu doğurduğunu ve i'nin değerini 1 arttırdığını en başta anımsatayım

//sayı 6'ya geldiğinde, döngüyü durduran bir for döngüsü örneği:

fn main() {
  for sayi in 0..8 {
    print!("{}", sayi);
    if sayi == 6 {
      break;
    }
  }
}

//0123456 çıktısını aldık

//-------------------------------------------------

//sayı 6'ya geldiğinde, döngüyü durduran bir while döngüsü örneği:

fn main() {
  let mut sayi = 0;
  let durum = true;
  while durum {
    print!("{}", sayi);
    if sayi == 6 {
      break;
    }
    sayi += 1;
  }
}

//0123456 çıktısını aldık

//-------------------------------------------------

//sayı 6'ya geldiğinde, döngüyü durduran bir loop döngüsü örneği:

fn main() {
  let mut sayi = 0;
  loop {
    print!("{}", sayi);
    if sayi == 6 {
      break;
    }
    sayi += 1; // sayi = sayi + 1;
  }
}

//0123456 çıktısını aldık
