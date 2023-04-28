mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                season_fruit: String::from("peaches") 
            }
        }
    }
}
/*
    В рамках общего крейта back_of_house является доступным модулем
    Для структуры и методов внутри нее необходимо указывать доступность ключевым словом pub
    В частности для работы с внешними методами доступно только одно поле структуры - toast
    Метод структуры Breakfast принимает строку и возвращает экземпляр структуры с измененными полями
*/

pub fn eat_at_restaraunt() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("Id liken {} toast please", meal.toast);
}