extern crate google_drive3 as drive3;
use crate::Result;
use drive3::DriveHub;
use google_dns1::Dns;
use rlib::net::http::{hyper, hyper_rustls::HttpsConnector, oauth::Oauth2Config};
use std::default::Default;
use yup_oauth2::{ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

pub async fn drive_handle(cfg: Oauth2Config) -> Result<DriveHub> {
  let secret: ApplicationSecret = cfg.into();
  let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
    .build()
    .await
    .expect("failed to start authenticator");
  Ok(DriveHub::new(
    hyper::Client::builder().build(HttpsConnector::with_native_roots()),
    auth,
  ))
}

pub async fn dns_handle(_cfg: Oauth2Config) -> Result<Dns> {
  let secret: ApplicationSecret = Default::default();
  let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
    .build()
    .await
    .expect("failed to start authenticator");
  Ok(Dns::new(
    hyper::Client::builder().build(HttpsConnector::with_native_roots()),
    auth,
  ))
}

#[test]
async fn test() {
  pub const SECRET: &'static str =
    "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\
     \"client_secret\":\"UqkDJd5RFwnHoiG5x5Rub8SI\",\"token_uri\":\"https://accounts.google.\
     com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:\
     oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\
     \"14070749909-vgip2f1okm7bkvajhi9jugan6126io9v.apps.googleusercontent.com\",\
     \"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}";
}
