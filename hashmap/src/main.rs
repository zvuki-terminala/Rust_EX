fn main() {
    
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    //Обьявление хэш-мапы

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //Внесение значений в обьявленную хэш-мапу

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    /* 
        Получение значения из хэш-мапы через: .get (Выдает Object<&V>) .copied (Преобразовывает в Object<V>)
        .unwrap_or (Преобразовывает Null (в случае отсутствия значения) в 0) 
    */
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    println!("{}", &score);

    let text = "Hello demension world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    //Разбиваем ссылку на строку на слова между пробелов, передаем их в word -> в хэш

    println!("{:?}", map);
}

