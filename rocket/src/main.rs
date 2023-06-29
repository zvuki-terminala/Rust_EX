#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::fs::TempFile;
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

#[get("/world")] //Маршрут
fn world() -> &'static str {
    //Обработчик
    "Hello world!"
}
//Get запрос по данному маршруту сделает что то

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {}", seconds)
}
//Асинхронный маршрут

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| Error::new(ErrorKind::Interrupted, e))??;

    Ok(vec)
}
//Многозадачный маршрут - ожидает появления файла, пятисотит

#[get("/cool_bro/<name>/<age>/<cool>")]
fn cool_bro(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You cool {}-age {}", age, name)
    } else {
        format!("Lets talk abou your COOL {}", name)
    }
}
//Динамический путь с обработками

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
//Использование сегментов
//Защищенное от атак с обходом пути получение файла из каталога static

#[get("/foo/<_>/bar")]
fn foo_bar() -> &'static str {
    "Foo ________ bar"
}
#[get("/<_..>")]
fn everything() -> &'static str {
    "You are here"
}
//Игнорируемые сегменты - в первом случае ответ на Get запрос поступит при переходе по пути из двух известных
//сегментов и одного пропущенного
//Во втором случае Get запрос на любой путь дает ответ

//Запросы с защитой по ключу есть в документации по FromReques
//Пример обработки JSON файла в репе Rocket/examples

#[post("/upload", format = "plain", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    file.persist_to(permanent_location).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/std", routes![world])
        .mount("/async", routes![delay])
        .mount("/blocking", routes![blocking_task])
        .mount("/dynamic", routes![cool_bro])
        .mount("/mpl_segments", routes![files])
        .mount("/ign_segments", routes![foo_bar])
        .mount("/ign_segments", routes![everything])
        .mount("/tmp", routes![upload])
}
//Монтируем маршруты к базовым путям или /
