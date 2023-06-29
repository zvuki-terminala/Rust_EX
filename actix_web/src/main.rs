use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use std::sync::Mutex;

struct AppState {
    app_name: String,
}
//Структура данных для описания состояния приложи

struct AppStateWithCounter {
    counter: Mutex<i32>,
}
/*
    Для создания счетчика запросов к хэндлеру используем мьютекс
    Мьютекс отдает данные одному потоку за раз, запрещая другим потокам
    читать из него данные при помощи функции .lock()
*/

//#[get("/")]
//async fn hello() -> impl Responder {
//    HttpResponse::Ok().body("Hello World")
//}
//Хэндлер на заданный маршрут

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}
//Хэндлер с задаваемым маршрутом внутри App блока

async fn index() -> impl Responder {
    "Hello world"
}

#[get("/index2")]
async fn index2(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}
//Передача данных состояния приложи в хэндлер
//Состояние может быть изменяемо если связать функцию
//web::Data с функцией внутри приложения

async fn index3(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {counter}")
}
/*
    Создаем функцию с параметром web::Data(Струтура с мьютексом) и возвращаемой строкой
    т.к структура в области видимости передаем в data мьютекс - блокируем его для
    других потоков и проверяем на ошибку
    Увеличиваем счетчик и печатаем при активации хэндлера
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Все функции Actix асинхронны

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    /*
        Для задания стартового состояния хэндлера определяем его переменные в main
        Замыкание HttpServer является локальной областью для хэндлеров, если не определить счетчик снаружи
        в main, а внутри фабрики, состояние не будет меняться при повторном запросе к хэндлеру
    */
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        //Переносим права на владение counter из верхней области в замыкатель при помощи move
        //Структура HttpServer
        App::new() //Фабрика
            //.service(hello) //Хэндлеры с прописанным маршрутом вызываются при помощи
            .service(echo) //service, без маршрута, при помощи route
            .route("/hey", web::get().to(manual_hello)) // ("/Маршрут", web::Запрос().to(Хэндлер-функция))
            .service(
                web::scope("/app") //Группа ресурсов
                    .route("index.html", web::get().to(index)), //Хэндлер доступный только в /app
            )
            .app_data(web::Data::new(AppState {
                //Передача данных из строки в .app_data
                app_name: String::from("Actix Web"),
            })) //Данное состояние не является глобальным для сервера
            .service(index2)
            .app_data(counter.clone()) //Клонируем значение счетчика в .app_data и ин7ициализируем новое состояние
            .route("/", web::get().to(index3))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))? //Прослушка
    .run() //Ран
    .await //Асинхрон
}
