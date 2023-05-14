//assignment atama 
//değer ataması yapılıyor = ile, eşitlik değil!

fn main() {
   let x = 5;
   let y = x;
   println!("y = x");
   println!("x'in değeri : {}", x);
   println!("y'nin değeri : {}", y);
}

/*
çıktı:
y = x
x'in değeri : 5
y'nin değeri : 5
*/

fn main() {
    let mut a = 5;
    println!("a:{}", a);
    a += 1;
    println!("a+=1:{}", a); // a = a + 1
    println!("a:{}", a);
    a -= 1;
    println!("a-=1:{}", a); // a = a -1
    println!("a:{}", a);
    a /= 5;
    println!("a/=5:{}", a); // a = a / 5
    println!("a:{}", a);
    a *= 3;
    println!("a*=3:{}", a); // a =  a * 3
}

/*
a:5
a+=1:6
a:6
a-=1:5
a:5
a/=5:1
a:1
a*=3:3
*/
