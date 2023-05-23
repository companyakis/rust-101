//borrowing ve dereferencing -> rust için zor konular

//heap ve stack konularına bakmakta yarar var
//ilerleyen konularda bu bölümlere daha ayrıntılı örnekler vereceğiz

//shared borrowing -> & kullanacağız, yalnızca okuma
//mutable borrowing -> & mut kullanacağız, okuma ve değiştirme

fn main() {
    let a = 5;
    let mut b = 6;
    
    //yalnızca okuma 
    let c = &a;
    println!("c:{}", c);
    //a değeri aynı kalacak
    println!("a:{}", a);
    
    //okuma ve değiştirme
    let d = &mut b;
    println!("d:{}", d);
    *d = 12; //dereferencing konusu!
    println!("yeni d değeri: {}", d);
    println!("yeni b değeri: {}", b);
}

/*
c:5
a:5
d:6
yeni d değeri: 12
yeni b değeri: 12
*/

//bir örnek daha yapalım. konular karışık gelebilir:) ama rust biraz böyle:(

fn main() {
    let mut x = 35;
    println!("x ilk değeri:{}", x);
    let y = & mut x;
    println!("y ilk değeri:{}", y);
    *y = 141;
    println!("y son değeri:{}", y);
    println!("x son değeri:{}", x);
}

/*
x ilk değeri:35
y ilk değeri:35
y son değeri:141
x son değeri:141
*/
