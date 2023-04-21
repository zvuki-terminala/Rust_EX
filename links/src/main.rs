fn main() {
    let s1 = String::from("Hello");
    let len = calculate_lenght(&s1);
    println!("The lenght of '{}' is {}.", s1, len);

    let mut s2 = String::from("Hello");
    change(&mut s2);
    println!("{}", s2);

    let mut s3 = String::from("Hello");
    let r1 = &s3;
    let r2 = &s3;
    println!("ptr {} and ptr2 {}", r1, r2);
    /* 
        В одной области видимости можно создать несколько неизменяемых переменных
        со значением ссылки, если появляется мутабельная ссылка, начинается гонка
        данных в области видимости, изменяемая ссылка может существовать вне области
        видимости неизменяемых ссылок 
    */
    let r3 = &mut s3;
    println!("Mutable link for s3 in out scope {}", r3);
    /*
        Область видимости r1 r2 окончилась на их применении в println
        Поэтому мы можем создать изменяемую ссылку вне области и
        использовать ее, если вызвать значения r1 и r2 за вызовом r3 
        это увеличит их обасть видимости тем самым создав гонку данных 
    */
    /* 
        let mut s4 = String::from("Hello World");
        let hello = &s4[0..5];
        let world = &s4[6..11];

        Срез строки - ссылка на строку по определенным позициям
    */
}
fn calculate_lenght(s: &String) -> usize {
    s.len()
}
/*
    Область видимости s1 - main(){область}, данный код передает значение переменной за область видимости
    при помощи ссылки. Передавая ссылку в функцию тип тоже должен быть ссылочным.
    Изменение значения ссылки невозможно если ссылка не мутабельна.
*/
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
/*
    Мутабельные переменные и мутабельные ссылки позволяют изменять значение ссылки вне области видимости
    и по возвращении в нее изменять значение переменной

    В один момент времени, может существовать либо одна изменяемая ссылка, либо любое количество неизменяемых ссылка,
    Все ссылки должны быть действительными
*/

