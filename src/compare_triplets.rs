//compare the triplets
fn main() {
    let a = [0,0,0,0,0];
    let b = [3,6,10,-6,32];
     //index,alice and bob change values. use mut
    let mut index: usize  = 0 ;
    let mut alice: i8 = 0;
    let mut bob: i8 = 0;
     //checking length of Array
    if a.len()!= b.len(){
        panic!("A and B must have the samen length!" );

    }
    while index < a.len() {
        // println!("value  a is :{}",a[index] );
        // println!("the value b is {}",b[index] );
        if a[index] >  b[index]{
             alice = alice + 1;
        }
         else if a[index] < b[index] {
            bob =  bob + 1;
        }
        index = index + 1;
    }
    println!("Alice has: {} points", alice);
    println!("Bob has {} points", bob );
}
