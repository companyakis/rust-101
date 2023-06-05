//borrowing ve slicing -> dilimleme

//vector/array dilimleme, istenen parçaları alma
//örnek array için, ama vector için de benzer kullanım

fn main () {
    let arr_1:[i32;6] = [-101, 101, 202, 303, 404, 505];
    
    let odunc_dilim = &arr_1[0..3];
    
    println!("{:?}", arr_1);
    
    println!("{:?}", odunc_dilim);
}

/*
[-101, 101, 202, 303, 404, 505]
[-101, 101, 202]
*/

//string dilimleme, ödünç

fn main () {
    let cumle = "Rust 101".to_string();
    
    let cumle_odunc = &cumle[0..4];
    
    println!("{}", cumle);
    
    println!("{}", cumle_odunc);
}

/*
Rust 101
Rust
*/

