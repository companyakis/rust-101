//borrowing konusu, borç, ödünç

//örnek 1: paylaşım, shared borrows

fn main () {
    //paylaşılan ödünç verme, shared borrow
    
    let a = 101;
    
    let b = &a; //& ile ödünç
    
    let c = "Günaydın!".to_string();
    
    let d = &c;
    
    println!("{}", b); //101
    
    println!("{}", a); //101
    
    println!("{}", d); //Günaydın!
     
    println!("{}", c); //Günaydın!
}

//örnek 2 mutable borrow

fn main () {
    let mut a = 101;
    
    let b = &mut a;
    
    println!("{}", b); //101
    
    println!("{}", a); //101
    
    let c = &mut a;
    
    println!("{}", c); //101
    
    *c = 202;
    
    println!("{}", c); //202
    
    println!("{}", a); //a için de 202!
}
