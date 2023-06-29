use std::{
    fs::File,
    io::{ErrorKind, Read, self}
};
fn main() {
    //panic!("crash and burn");
    /*
        Обратная трассировка ошибки вызывается
        переменной среды RUST_BACKTRACE со значением 1
    */

    //Обработка ошибок при помощи вложенных match
    let greeting_file_result = File::open("Hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    /*
        обработка ошибок при помощи сопоставления типа Result<T, E> - если ОК идем дальше,
        если возвращается error проверяем соответствие с предполагаемым error-kind
        и исправляем ситуацию, если же вмешательство не помогает,
        вызываем panic!
    */
    let _greeting_file2 = File::open("Hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    /*
        Достаточно простой способ указания ошибки - но не подходит для создания
        управляемой паники
    */
    let _greeting_file3 =
        File::open("Hello3.txt").expect("Hello3.txt should be included in this project");


    
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("Users.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file2 = File::open("Users2.txt")?;
    let mut username2 = String::new();
    username_file2.read_to_string(&mut username2)?
    Ok(username2)
}
