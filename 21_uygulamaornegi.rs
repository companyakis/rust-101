//buraya kadar işlediğimiz konulara bir örnek verelim ve yeni konulardan önce dinlenmiş olalım

fn main() {
    //iki sayını toplamının dördüncü kuvvetini bulma örneği yapalım.
    //başka yollarla da yapılabilir, ama burada pow işlevini görelim
    
    let a = 12;
    let b= 23;
    
    //(a+b) üzeri 4 kaça eşittir?
    //pow kullanmadan önce, sonucun veri türünü de ekliyoruz. u64 zorunlu değil! büyük sayılar kullanacaksak, veri türünü dikkatli seçelim...
    
    let toplam = a + b;
    
    let sonuc = u64::pow(toplam, 4); //4 ile kaçıncı derece olduğunu belirttik
    
    println!("({}+{})^4 = {}", a, b, sonuc);
}

// (12+23)^4 = 1500625

//35 * 35 * 35 * 35
