use actix_web::{HttpRequest, HttpResponse, Responder};
use bytes::Bytes;
use std::env;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

async fn read_mp4_fragment(offset: u64, length: usize) -> Result<Bytes, std::io::Error> {
    println!("read mp4 frag, offset, length {}", length);
    let mut file = File::open("server/assets/test.mp4").await?;

    file.seek(std::io::SeekFrom::Start(offset)).await?;
    println!("file seek");

    let mut buffer = vec![0; length];
    file.read_exact(&mut buffer).await?;
    Ok(Bytes::from(buffer))
}

// async fn get_fragment(req: HttpRequest) -> impl Responder {
//     let query = req.query_string();
//     let offset: u64 = query
//         .split('&')
//         .find(|&s| s.starts_with("offset="))
//         .and_then(|s| s.split('=').nth(1))
//         .and_then(|s| s.parse().ok())
//         .unwrap_or(0);
//     let length: usize = query
//         .split('&')
//         .find(|&s| s.starts_with("length="))
//         .and_then(|s| s.split('=').nth(1))
//         .and_then(|s| s.parse().ok())
//         .unwrap_or(1024 * 1024); // Default to 1MB

//     match read_mp4_fragment(offset, length).await {
//         Ok(bytes) => HttpResponse::Ok().body(bytes),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

#[actix_web::get("api/fragment")]
async fn fragment(req: HttpRequest) -> impl Responder {
    let query = req.query_string();
    println!("this is query: {}", query);
    let offset: u64 = query
        .split('&')
        .find(|&s| s.starts_with("offset="))
        .and_then(|s| s.split('=').nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let length: usize = query
        .split('&')
        .find(|&s| s.starts_with("length="))
        .and_then(|s| s.split('=').nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(1024 * 1024); // Default to 1MB

    println!("LENGTH: {}", length);

    match read_mp4_fragment(offset, length).await {
        Ok(bytes) => HttpResponse::Ok().body(bytes),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
