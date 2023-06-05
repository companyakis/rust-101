//hafıza yönetimi copy type and moved type

fn main () {
    let x = 101;
    
    let y = x; // x'in kopyası oluşturuldu, sahiplik devredildi ama 2'si de kullanımda
    
    println!("{}", x); //101
    println!("{}", y); //101
}

//yukarıda görüldüğü üzere, x ve y kullanımda

//strings ve vectors için durum farklı, moved type 

fn main () {
    let a = "Sahibim değişecek:(".to_string();
    
    println!("{}", a);
    
    let b = a; //a'nın değeri b'ye taşındı
    
    println!("{}", b); //hata almayız
    
    eprintln!("{}", a); //bu yazdırma  işlemi hata verir, çünkü sahiplik taşındı, hafıza yönetimi!
}

//vector örneği hiç yapmadık

fn main () {
    let a = vec![3, 3, 3, 3, 3];
    
    let b = a; //a'nın değeri b'ye taşındı
    
    println!("{:?}", b); //[3, 3, 3, 3, 3]
    
    println!("{:?}", a); //hata alırız
}
