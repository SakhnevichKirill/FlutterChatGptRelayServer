# OpenAI API Relay server 
OpenAI API Relay server for [ChatGPT AI flutter App](https://github.com/SakhnevichKirill/FlutterChatGpt)

Мультиплатформенное приложение, позволяющее пользоваться ChatGPT без ограничений и VPN.

## Функционал

- [x] Передача запроса с помощью API ChatGPT
- [x] Получение потока ответа 
- [x] Общение клиент-сервиса с помощью сокетов
- [x] Адаптивный дизайн под разные платформы

## Возможности пользователя

* Создание нового чата
* Вопросы ChatGPT версии 3.5 и получение ответа в контексте диалога
* В дальнейшем мы планируем добавить возможность разных промптов пользователем, таких как доктор, комик, создатель промптов, тренер и многих других.

## Технологии

* Rust
* Flutter
* Docker
* Gradle

## Зависимости


> [dependencies]
> 
> axum = { version = "0.6.18", features = ["ws", "headers"] }
>
> chatgpt_rs = { version = "1.1.8", features = ["streams"] }
> 
> chrono = "0.4.26"
> 
> dotenvy = "0.15.7"
> 
> futures-util = "0.3.28"
> 
> serde = { version = "1.0.163", features = ["derive"] }
> 
> serde_json = "1.0.96"
> 
> tokio = { version = "1.28.2", features = ["full"] }
> 
> tokio-tungstenite = "0.19.0"
> 
> csv= "1.1.6"
> 
> lazy_static = "1.4.0"

## Запуск сервера

>cargo run

## Ссылки

* [Библиотека подключения к API ChatGPT на Rust](https://github.com/Maxuss/chatgpt_rs/blob/master/examples/streamed_conversation.rs)
* [Пример приложения на Flutter](https://github.com/wewehao/flutter_chatgpt/blob/main/lib/page/ChatPage.dart)
* [Промпты](https://habr.com/ru/articles/528116/)

