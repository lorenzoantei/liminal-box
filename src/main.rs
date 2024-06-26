use actix_web::{get, Responder};
#[cfg(feature = "ssr")]
use actix_web::{Error, HttpRequest, HttpResponse, body::MessageBody, dev::{ServiceRequest, ServiceResponse}};
#[cfg(feature = "ssr")]
use actix_multipart::MultipartError;
#[cfg(feature = "ssr")]
use leptos::*;
#[cfg(feature = "ssr")]
use actix_web_lab::middleware::{from_fn, Next};

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use actix_multipart::form::{MultipartFormConfig, tempfile::TempFileConfig};
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use shareboxx::app::*;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .wrap(from_fn(domain_redirect))
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(10 * 1024 * 1024 * 1024) // 10 GB
                    .memory_limit(10 * 1024 * 1024) // 10 MB
                    .error_handler(handle_multipart_error),
            )
            // Leptos server side API
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // SSE events to notify clients of new chat messages
            .service(counter_events)
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            .service(Files::new("/files", "./files"))
            // serve the favicon from /favicon.ico
            .service(favicon)
            // serve the favicon from /apfelgrotezk-fett.woff2
            .service(apfelgrotezk)
            // // uploader
            // .service(web::resource("/upload").route(web::post().to(save_files)),
            // )
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
            // Store temp files on same drive, otherwise .persist() will fail due to cross-device link error
            .app_data(TempFileConfig::default().directory("files"))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
fn handle_multipart_error(err: MultipartError, _req: &HttpRequest) -> Error {
    logging::log!("Multipart error: {}", err);
    return Error::from(err);
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}
#[actix_web::get("apfelgrotezk-fett.woff2")]
async fn apfelgrotezk(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/apfelgrotezk-fett.woff2"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use shareboxx::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}


// #[cfg(feature = "ssr")]
// #[derive(Debug, actix_multipart::form::MultipartForm)]
// struct UploadForm {
//     #[multipart(rename = "file")]
//     files: Vec<actix_multipart::form::tempfile::TempFile>,
//     upload_path: actix_multipart::form::text::Text<String>,
// }

// #[cfg(feature = "ssr")]
// async fn save_files(
//     actix_multipart::form::MultipartForm(form): actix_multipart::form::MultipartForm<UploadForm>,
// ) -> Result<impl actix_web::Responder, actix_web::Error> {

//     for f in form.files {
//         let path = format!("./files/{}{}", form.upload_path.clone(), f.file_name.unwrap());
        
//         // Check if file already exists, if so, append a number to the filename in front of the extension
//         let mut new_path = path.clone();
//         let mut i = 1;
//         let basepath = std::path::Path::new(&path).parent().unwrap().to_str().unwrap();
//         let file_basename = std::path::Path::new(&path).file_stem().unwrap().to_str().unwrap();
//         let file_extension = std::path::Path::new(&path).extension().unwrap().to_str().unwrap();
//         while std::path::Path::new(&new_path).exists() {
//             logging::log!("Uploaded file {} already exists, trying to save as new file {}-{}.{}", new_path, file_basename, i, file_extension);
//             new_path = format!("{}/{}-{}.{}", basepath, file_basename, i, file_extension);
//             i += 1;
//         }

//         f.file.persist(new_path).unwrap();
//     }

//     Ok(actix_web::web::Redirect::to("/").see_other())
// }

#[cfg(feature = "ssr")]
async fn domain_redirect(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Check if request hostname matches shareboxx.lan, otherwise redirect to it.
    if req.connection_info().host() != "shareboxx.lan" && !req.connection_info().host().starts_with("127.0.0.1") {
        return Ok(ServiceResponse::new(
            req.request().to_owned(),
            HttpResponse::TemporaryRedirect()
                .append_header(("Location", "https://shareboxx.lan"))
                .finish(),
        )
        .map_into_boxed_body());
    }

    next.call(req).await.map(ServiceResponse::map_into_boxed_body)
}

#[cfg(feature = "ssr")]
#[get("/ws")]
async fn counter_events() -> impl Responder {
    use actix_web::web;
    use shareboxx::app::ssr_imports::*;
    use futures::StreamExt;

    let stream = futures::stream::once(async {
        shareboxx::app::get_message_count().await.unwrap_or(0)
    })
    .chain(COUNT_CHANNEL.clone())
    .map(|value| {
        Ok(web::Bytes::from(format!(
            "event: message\ndata: {value}\n\n"
        ))) as Result<web::Bytes, Error>
    });
    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .streaming(stream)
}
