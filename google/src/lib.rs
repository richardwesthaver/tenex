//! lib.rs --- Google Cloud Async APIs
/**
- generated from Discovery Service API Docs: https://developers.google.com/discovery/v1/reference/apis/list
- via https://github.com/dermesser/async-google-apis
*/
pub use common;

pub mod drive;

/// Create a new HTTPS client.
pub fn https_client() -> common::TlsClient {
  let conn = hyper_rustls::HttpsConnector::with_native_roots();
  let cl = hyper::Client::builder().build(conn);
  cl
}

#[cfg(feature = "google")]
#[tokio::test]
async fn test_drive() {
  let https = https_client();
  let sec = common::yup_oauth2::read_application_secret("client_secret.json")
    .await
    .expect("client secret couldn't be read.");
  let auth = common::yup_oauth2::InstalledFlowAuthenticator::builder(
    sec,
    common::yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
  )
  .persist_tokens_to_disk("tokencache.json")
  .hyper_client(https.clone())
  .build()
  .await
  .expect("InstalledFlowAuthenticator failed to build");

  let scopes = vec![drive::DriveScopes::Drive];
  let mut cl = drive::FilesService::new(https, Arc::new(auth));
  cl.set_scopes(&scopes);

  let arg = std::env::args().skip(1).next();
  if let Some(fp) = arg {
    upload_file(&cl, Path::new(&fp))
      .await
      .expect("Upload failed :(");
  } else {
    // By default, list root directory.
    let mut general_params = drive::DriveParams::default();
    general_params.fields = Some("*".to_string());
    let mut p = drive::FilesListParams::default();
    p.q = Some("'root' in parents and trashed = false".to_string());
    p.drive_params = Some(general_params);

    let resp = cl.list(&p).await.expect("listing your Drive failed!");

    if let Some(files) = resp.files {
      for f in files {
        println!(
          "{} => {:?}",
          f.id.unwrap(),
          f.name.as_ref().unwrap_or(&"???".to_string()),
        );
      }
    }
  }
}
