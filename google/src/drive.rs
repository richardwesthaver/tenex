mod drive_v3_types;
use drive_v3_types as drive;

use common;
use std::path::Path;

/// Upload a local file `f` to your drive.
pub async fn upload_file(cl: &drive::FilesService, f: &Path) -> anyhow::Result<()> {
    let fname = f.file_name().unwrap().to_str().unwrap();

    let mut general_params = drive::DriveParams::default();
    general_params.fields = Some("*".to_string());

    let mut params = drive::FilesCreateParams::default();
    params.drive_params = Some(general_params.clone());
    params.include_permissions_for_view = Some("published".to_string());
    let mut file = drive::File::default();
    file.name = Some(fname.to_string());

    // Upload data using the upload version of create(). We obtain a `File` object.
    //
    let resumable = cl.create_resumable_upload(&params, &file).await?;

    let tokiofile = tokio::fs::OpenOptions::new().read(true).open(f).await?;
    let resp = resumable.upload_file(tokiofile).await?;

    // If you used this method, no file content would be uploaded:
    // let resp = cl.create(&params, &file).await?;
    println!(
        "Uploaded file: {} (id = {}) with metadata: \n {:?}",
        resp.name.as_ref().unwrap(),
        resp.id.as_ref().unwrap(),
        resp
    );

    let file_id = resp.id.unwrap();

    // Now get the file and check that it is correct.
    //
    let mut params = drive::FilesGetParams::default();
    params.file_id = file_id.clone();
    params.drive_params = Some(general_params.clone());
    // This parameter will trigger the download.
    params.drive_params.as_mut().unwrap().alt = Some(drive::DriveParamsAlt::Media);
    let mut dest = vec![0 as u8; 128];

    // Get file contents.
    let get_file = cl.get(&params).await?.do_it_to_buf(&mut dest).await?;
    println!("{:?}", get_file);

    // get() returns a response type (Response(...)) if the server returned an application/json
    // response, otherwise it will download to a Writer. If you don't supply a Writer and there is
    // data to download, an error is returned (ApiError::DataAvailableError).
    match get_file {
        common::DownloadResult::Response(_) => {
            // We want download! No metadata.
            assert!(false);
        }
        common::DownloadResult::Downloaded => println!("Download success."),
    }

    // Get file metadata. We don't specify &alt=media, so no download will take place.
    let mut params = drive::FilesGetParams::default();
    params.file_id = file_id.clone();
    let get_file = cl.get(&params).await?.do_it(None).await?;
    println!("{:?}", get_file);

    match get_file {
        common::DownloadResult::Response(gf) => {
            println!("Metadata success.");
            assert!(gf.name == Some(fname.to_string()));
        }
        // We want metadata! No download.
        common::DownloadResult::Downloaded => assert!(false),
    }
    Ok(())
}
