//vector üyeleri arasında yineleme , iteration

//.iter() işlevi

fn main() {
    let vec = vec![1, 3, 3, 4, 7, -1];
    let mut dizin = 0; //index=dizin
    for uye in vec.iter() {
        println!("Dizin: {} ve üye: {}", dizin, uye);
        dizin += 1;
    }
}

/*
Dizin: 0 ve üye: 1
Dizin: 1 ve üye: 3
Dizin: 2 ve üye: 3
Dizin: 3 ve üye: 4
Dizin: 4 ve üye: 7
Dizin: 5 ve üye: -1
*/

//yineleme sırasında üyeleri değiştirme -> .iter_mut() işlevi

fn main() {
   let mut vec = vec![1, 2, 3, 4, 5, -22];
   println!("Vector başlangıç durumu : {:?}", vec);
   for x in vec.iter_mut(){ //her bir üyenin 2 katının 1 eksiğini elde edelim
       *x = *x * 2 - 1; //dereferencing mi:) 
   }
   println!("Güncellenen vector : {:?}", vec);
}

/*
Vector başlangıç durumu : [1, 2, 3, 4, 5, -22]
Güncellenen vector : [1, 3, 5, 7, 9, -45]
*/

//vector dilimleme -> slicing
//let slice_vec:&[data_type] = &our_vector[start..end]

fn main() {

   let vector = vec![-1, 0, 4, 6, -45];

   //son iki elemanı alalım mı

   let slice:&[i8] = &vector[3..5];

   println!("Vector son iki üye: {:?}", slice); //Vector son iki üye: [6, -45]
 }
