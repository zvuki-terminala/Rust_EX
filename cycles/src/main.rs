fn main() {
    let number = 69;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Not divisible!");
    }

    let condition = false;
    let number_two = if condition { 5 } else { 6 };

    println!("The value of number is: {number_two}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    /*
        В данном примере значение получаемое в loop цикле присваивается переменной result указанной в начале
        т.к. в таком случае использования loop находится в составе конструкции то он должен оканчиваться 
        на ";" т.к. в этом случае становится инструкцией
    */

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
    /*
        Конструкция типа 'test является меткой цикла, можно использовать для операций над циклом из под внутренних циклов
        или при помощи функций. К метке применимы операторы управления циклами по типу break continue
    */

    let mut number3 = 3;

    while number3 != 0 {
        println!("{number3}!");

        number3 -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
    
    for number4 in (1..4).rev() {
        println!("{number4}");
    }
    println!("LIFTOFF!!!");

}
