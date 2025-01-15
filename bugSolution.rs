fn main() {
    let mut x = 5;

    // Solution 1: Cloning
    let y = x.clone();
    let mut z = x;
    z += 1;
    y += 1;
    x = z; 
    println!("x = {}", x); 

    // Solution 2: Using a mutable reference in a block
    { 
        let z = &mut x; 
        *z += 1; 
    }
    x += 1;
    println!("x = {}", x);

    //Solution 3: Using RefCell or other interior mutability
    use std::cell::RefCell;
    let x = RefCell::new(5);
    *x.borrow_mut() += 1;
    *x.borrow_mut() += 1; 
    println!("x = {}", x.borrow());
}