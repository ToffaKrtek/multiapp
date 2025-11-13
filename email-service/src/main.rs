// src/main.rs
use std::collections::HashMap;
use std::sync::Arc;

use handlebars::Handlebars;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use tokio;
use tonic::{Request, Response, Status, transport::Server};

// Импортируем сгенерированные типы
pub mod email_service {
    tonic::include_proto!("email_service");
}
use email_service::{
    SendEmailRequest, SendEmailResponse,
    email_service_server::{EmailService, EmailServiceServer},
};

// Конфигурация SMTP (лучше вынести в конфигурационный файл или env vars)
const SMTP_HOST: &str = "smtp.gmail.com"; // Замените на ваш SMTP-сервер
const SMTP_PORT: u16 = 587;
const SMTP_USERNAME: &str = "your_email@gmail.com"; // Замените
const SMTP_PASSWORD: &str = "your_app_password"; // Используйте App Password!

// Структура для нашего сервиса, содержащая шаблоны и SMTP-транспорт
pub struct MyEmailService {
    pub handlebars: Arc<Handlebars<'static>>,
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
}

// Реализуем gRPC сервис
#[tonic::async_trait]
impl EmailService for MyEmailService {
    async fn send_email(
        &self,
        request: Request<SendEmailRequest>,
    ) -> Result<Response<SendEmailResponse>, Status> {
        let req = request.into_inner();

        // Используем встроенный шаблон, так как у нас только один шаблон
        let email_body = format!(
            "Ваш email был зарегистрирован в системе. Код для доступа {}, он будет валидным в течение 3 часов с момента выпуска.",
            req.code
        );

        // Создаем сообщение
        let email = Message::builder()
            .to(req
                .to_email
                .parse()
                .map_err(|_| Status::invalid_argument("Invalid email address"))?)
            .from(SMTP_USERNAME.parse().unwrap()) // Проверка валидности email отправителя
            .subject("Your Verification Code") // Тема может быть также параметризована
            .header(ContentType::TEXT_HTML) // Или TEXT_PLAIN
            .body(email_body)
            .map_err(|e| {
                eprintln!("Failed to build email: {}", e);
                Status::internal("Failed to build email")
            })?;

        // Отправляем сообщение
        match self.mailer.send(email).await {
            Ok(_) => {
                println!("Email sent successfully to {}", req.to_email);
                Ok(Response::new(SendEmailResponse {
                    success: true,
                    message: "Email sent successfully".into(),
                }))
            }
            Err(e) => {
                eprintln!("Failed to send email: {}", e);
                Ok(Response::new(SendEmailResponse {
                    success: false,
                    message: format!("Failed to send email: {}", e),
                }))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализируем tracing для логирования
    tracing_subscriber::fmt::init();

    // Настройка Handlebars
    let mut handlebars = Handlebars::new();
    // Регистрируем шаблоны. Путь к папке с шаблонами.
    // Предположим, шаблоны находятся в папке `templates` в корне проекта.
    handlebars.register_template_string("email_template", "Ваш email был зарегистрирован в системе. Код для доступа {{code}}, он будет валидным в течении 3 часов с момента выпуска.")?;

    // Создаем асинхронный SMTP-транспорт
    let creds = Credentials::new(SMTP_USERNAME.into(), SMTP_PASSWORD.into());
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(SMTP_HOST)
        .expect("Ошибка при установке SMTP relay")
        .credentials(creds)
        .build();

    // Создаем экземпляр нашего сервиса
    let email_service = MyEmailService {
        handlebars: Arc::new(handlebars),
        mailer,
    };

    let addr = "0.0.0.0:50051".parse().unwrap(); // gRPC порт
    println!("Email Service Listening on {}", addr);

    Server::builder()
        .add_service(EmailServiceServer::new(email_service))
        .serve(addr)
        .await?;

    Ok(())
}
