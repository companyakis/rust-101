//vector yazdırma -> döngü (loop) ya da debug trait {:?} ile 

fn main() {   
    let bir_vector = vec![-1, 0, 1, 2];

    println!("{:?}", bir_vector);

    let mut bir_vector_index = 0;
    for uye in bir_vector {
        println!("Vector dizini {:?} ve üyesi {:?}", bir_vector_index, uye);
        bir_vector_index = bir_vector_index + 1;
    }
}
/*
[-1, 0, 1, 2]
Vector dizini 0 ve üyesi -1
Vector dizini 1 ve üyesi 0
Vector dizini 2 ve üyesi 1
Vector dizini 3 ve üyesi 2
*/
