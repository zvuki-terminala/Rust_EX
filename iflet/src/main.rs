#[derive(Debug)]
enum UnState {
    Alabama,
    Alaska,    
}
#[derive(Debug)]
enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UnState),
}

fn main() {
    /*
        Данный код выполняется тоглько в случае если внутри переменной Some
        находится значение подходящее типу u8
    */
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // Можно упростить код избежав применения пустого шаблона
    let config_max2 = Some(3u8);
        if let Some(max2) = config_max2 {
            println!("The maximum is configured to be {}", max2);
        }
    /*
        Преимущества второго подхода при создании ветвления в том,, что функция так же не
        выполнится в случае несоответствия значения, но при этом мы избегаем синтаксический сахар
        
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }

        Этот пример взят из крейта про перечисления - можно обычным методом передать значения поля перечисления
        в функцию и сделать ветвление, при помощи if let можно облегчить конструкцию и описать if else 
        ветки которого выполняют действие только в случае соответствия указанного внутри функции типа с тем 
        который был передан в качестве параметра в main()
    */
    coin_state(Coin::Quarter(UnState::Alaska));//первая ветка
    coin_state(Coin::Nickel); //вторая ветка
}

//Функция принимает обьекты перечисления
fn coin_state(coin: Coin) {
    let mut count2 = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state); //Если в функцию передано значение перечисления соответствующее
                                                    //типу Coin::Quarter выполняется первая ветка
    }
    else {
        count2 += 1;                                //Если передается другое, например Coin::Nickel выполняется
        println!("Count {}", count2);               //вторая ветка
    }
}
// Основное применение - если match дает много веток проще использовать if let конструкции