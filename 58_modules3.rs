//iç içe module, nested module, uygulama örneği yapalım

//dış module tanımlama
mod outer_module {
    fn first_function() {
        println!("Ben gizli bir işlev tanımladım.");
    }
    
    //iç içe module, nested module
    pub mod inner_module {
        //içteki işlev
        pub fn second_function() {
            println!("İç içe module tanımlama örneği yaptık.");
            //outer module'ün private işlevini kullanacağız, "super" parent/outer module'e vurgu
            super::first_function();
        }
    }
}

fn main() {
    //yalnızca en içteki işlevi çağıralım mı? second_function()
    println!("En içteki second_function buraya geldi!");
    
    outer_module::inner_module::second_function();
}

/*
En içteki second_function buraya geldi!
İç içe module tanımlama örneği yaptık.
Ben gizli bir işlev tanımladım.
*/
