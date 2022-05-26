use lettre::{
    error::Error as EmailError,
    message::{header, Mailbox, MultiPart, SinglePart},
    transport::smtp::{authentication::Credentials, Error as SmtpError, SUBMISSIONS_PORT},
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

pub struct EmailAddress {
    pub name: Option<String>,
    pub email: String,
}

impl EmailAddress {
    pub fn new(name: Option<&str>, email: &str) -> EmailAddress {
        EmailAddress {
            name: name.map(str::to_owned),
            email: email.to_owned(),
        }
    }
}

impl Clone for EmailAddress {
    fn clone(&self) -> Self {
        EmailAddress {
            name: self.name.to_owned(),
            email: self.email.to_owned(),
        }
    }
}

impl From<EmailAddress> for Mailbox {
    fn from(email_address: EmailAddress) -> Self {
        Mailbox {
            name: email_address.name,
            email: email_address.email.parse::<Address>().unwrap(),
        }
    }
}

pub enum MessageBody {
    Text(String),
    Html(String),
}

#[derive(Debug)]
pub enum MailError {
    MissingSMTPConfiguration(String),
    SmtpError(SmtpError),
    EmailError(EmailError),
}

pub struct MailService {}

impl Clone for MailService {
    fn clone(&self) -> Self {
        MailService {}
    }
}

impl MailService {
    fn get_mailer() -> Result<AsyncSmtpTransport<Tokio1Executor>, MailError> {
        let (smtp_server, smtp_port) =
            match (std::env::var("SMTP_SERVER"), std::env::var("SMTP_PORT")) {
                (Ok(smtp_server), Ok(smtp_port)) => (
                    smtp_server,
                    smtp_port.parse::<u16>().unwrap_or(SUBMISSIONS_PORT),
                ),
                _ => {
                    return Err(MailError::MissingSMTPConfiguration(String::from(
                        "Missing SMTP_SERVER and SMTP_PORT configuration",
                    )))
                }
            };
        let smtp_tls = std::env::var("SMTP_TLS") != Ok(String::from("false"));

        let credentials = match (
            std::env::var("SMTP_USERNAME"),
            std::env::var("SMTP_PASSWORD"),
        ) {
            (Ok(username), Ok(password)) => Some(Credentials::new(username, password)),
            _ => None,
        };

        let mailer_builder = if smtp_tls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)
                .map_err(MailError::SmtpError)?
                .port(smtp_port)
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(smtp_server).port(smtp_port)
        };

        if let Some(credentials) = credentials {
            Ok(mailer_builder.credentials(credentials).build())
        } else {
            Ok(mailer_builder.build())
        }
    }

    pub async fn send_email(
        from: EmailAddress,
        to: EmailAddress,
        subject: String,
        message_body: MessageBody,
    ) -> Result<(), MailError> {
        let email_builder = Message::builder()
            .from(from.clone().into())
            .reply_to(from.into())
            .to(to.into())
            .subject(subject);

        let email = match message_body {
            MessageBody::Text(body) => email_builder.body(body).map_err(MailError::EmailError)?,
            MessageBody::Html(html) => {
                email_builder
                    .multipart(
                        MultiPart::alternative() // This is composed of two parts.
                            .singlepart(
                                SinglePart::builder()
                                    .header(header::ContentType::TEXT_HTML)
                                    .body(String::from(html)),
                            ),
                    )
                    .map_err(MailError::EmailError)?
            }
        };

        let mailer = MailService::get_mailer()?;

        mailer.send(email).await.map_err(MailError::SmtpError)?;

        Ok(())
    }
}
