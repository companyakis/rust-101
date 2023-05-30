//Vector işlevleri -> methods

/*
1- yeni bir vector oluşturma -> Vec::new()
2- eleman ekleme-> .push()
3- sondan eleman silme -> .pop()
4- bir değerin olmasını sorgulama -> .contains(), & ile kullanım, borrowing
5- belirtilen dizindeki üyeyi silme -> .remove(index)
6- vector uzunluğunu sorgulama -> .len()
*/

fn main() {
   let mut vec = Vec::new();

   println!("{:?}", vec);

   vec.push(0);
   vec.push(1);
   vec.push(2);
   vec.push(-5);
   vec.push(4);

   println!("{:?}", vec);

   vec.pop();

   println!("{:?}", vec);

   println!("Vector içinde 5 değeri var mı: {}", vec.contains(&5));

   vec.remove(1);

   println!("{:?}", vec);

   println!("Vector son uzunluk: {}", vec.len());
}

/*
[]
[0, 1, 2, -5, 4]
[0, 1, 2, -5]
Vector içinde 5 değeri var mı: false
[0, 2, -5]
Vector son uzunluk: 3
*/
 
