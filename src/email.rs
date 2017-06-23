use lettre::transport::smtp::{SmtpTransportBuilder, SmtpTransport, SecurityLevel};
use super::Settings;
use error::Error;

pub fn connect(settings: &Settings) -> Result<SmtpTransport, Error> {
    let builder = SmtpTransportBuilder::new("smtp.gmail.com:587")?
        .encrypt()
        .security_level(SecurityLevel::AlwaysEncrypt)
        .credentials(&settings.username, &settings.password);
    Ok(builder.build())
}