//Стандартная структура
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//Кортежная структура из разных типов полей
struct User2(bool, String, String, u64);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("something_user"),
        email: String::from("something@mail.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("newnewnew@mail.com");
    println!("Active: {}\nUsername: {}\nEmail: {}\nSign in: {}", user1.active, user1.username, user1.email, user1.sign_in_count);
    /* 
        Передача значений полей из прошлого экземпляра структуры в новый
    */
    let user2 = User {
        active: user1.active,
        username: String::from("new_user"),
        email: String::from("new@mail.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("Active: {}\nUsername: {}\nEmail: {}\nSign in: {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    let user3 = User2(true, String::from("testuser"), String::from("test@mail.com"), 100);
    println!("Active: {}\nUsername: {}\nEmail: {}\nSign in: {}", user3.0, user3.1, user3.2, user3.3)
}
/* 
    Сокращенная передача аргументов в поле структуру внутри функции
    
    fn build_user(username: String, email: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
*/