/* 
    В данной главе идет разбор коллекций, в частности
    Векторы, строки и хэш-таблицы
*/

fn main() {
    let v: Vec<i32> = Vec::new();
    // Аннотация типа позволяет указать Rust какие типы мы хотим хранить в векторе
    // Инициализируется как стандартная строка

    let v2 = vec![1, 2, 3, 4, 5];
    // Если вектор опрелделяется сразу, аннотация типа не требуется, но должен использоваться макросс "!"

    let mut v3 = Vec::new();
    // Простое обьявление вектора

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    v3.push(9);
    v3.push(0);
    // Добавление значений в вектор
    
    for i in &v3 {
        println!("Vector num = {}", i);
    }
    //Чтение вектора по индексу

    let third = &v2[2];
    //    ^      ^      ^  ^
    //    |      |      |  |
    //  Индекс  Тип    Вектор и позиция
    /*
        При чтении вектора можно использовать переменную и пердавать в нее значения
        по указанной в [] позиции
        Нюансы: 
        1) Если указывается тип он должен быть ссылочным (&i32) т.к. переменная ссылка на значение коллекции
        2) Указываемая позиция (v2[2]) также должна являться ссылкой, дабы не изменять значение коллекции 
        и не передавать права владения от коллекции методу печати, хотя возможна мутабельная ссылка
    */
    println!("This is third element in vector: {}", third);

    let four: Option<&i32> = v2.get(100);
    match four {
        Some(four) => println!("This is four element in Vector: {}", four),
        None => println!("This is no four element"),
    } 
    /* 
        Получение значение вектора при помощи get
        1) Создаем Option переменную с типом-ссылкой &i32
        2) Вызываем get для вектора, с указанием индекса
        3) Проверяем Option при помощи match

        4) Преимущества метода:
            Передав в Option индекс несуществующего элемента 
            мы получим вывод None, что не приведет к панике
    */


}
