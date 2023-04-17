fn main() {
    //Константа - тип должен быть определен
    const Const: u32 = 50;

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    println!("The value of Const is: {Const}");
}