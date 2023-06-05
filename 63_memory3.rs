//moved type konusunda, sahipliğin gittiğini ve hata aldığımızı gördük

//bunu aşmanın/delmenin bir yolu var mı? 

//eğer sahiplik konusunda hata alıp duruyorsak ve yeterli tecrübemiz yoksa, .clone() işlevi kullanılabilir, çoğaltma işlemi...

fn main () {

    let a = vec![3, 3, 3, 3, 3];
    
    let b = a.clone(); //a'nın değeri çoğaltıldı, ama sahiplik uçmadı
    
    println!("{:?}", b); //[3, 3, 3, 3, 3]
    
    println!("{:?}", a); //hata almayacağız, [3, 3, 3, 3, 3]
}
