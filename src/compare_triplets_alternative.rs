use std::cmp::Ordering;

fn main() {
    let a = [0,0,0,0,0];
    let b = [3,6,10,-6,32];
     //index,alice and bob change values. use mut
    let mut alice: i8 = 0;
    let mut bob: i8 = 0;
     //checking length of Array
    if a.len()!= b.len(){
        panic!("A and B must have the samen length!" );
    }

    for (ai, bi) in a.iter().zip(&b) {
        // println!("value  a is :{}",a[index] );
        // println!("the value b is {}",b[index] );
        match ai.cmp(bi) {
            Ordering::Less => bob = bob + 1,
            Ordering::Greater => alice = alice + 1,
            Ordering::Equal => (),
        }
    }
    println!("Alice has: {} points", alice);
    println!("Bob has {} points", bob );
}
