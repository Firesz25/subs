use crate::state::AppState;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use entity::prelude::{Sub, SubActiveModel};
use futures::TryStreamExt as _;
use once_cell::sync::Lazy;
use sea_orm::{ActiveModelTrait, Set};
use std::{collections::HashMap, sync::Mutex};

pub fn route_file(cfg: &mut web::ServiceConfig) {
    cfg.route("/download", web::get().to(download));
    cfg.route("/uploads", web::post().to(uploads));
    cfg.route("/uploads_details", web::post().to(uploads_details));
}

async fn download(
    state: web::Data<AppState>,
    id: web::Query<i32>,
    req: HttpRequest,
) -> HttpResponse {
    return match Sub::find_by_id(id.into_inner())
        .one(&state.conn)
        .await
        .unwrap()
    {
        Some(sub) => NamedFile::open_async(sub.path)
            .await
            .unwrap()
            .into_response(&req),
        None => HttpResponse::NotFound().body("Not Found"),
    };
}

async fn uploads(state: web::Data<AppState>, mut payload: Multipart) -> HttpResponse {
    while let Some(mut field) = payload.try_next().await.unwrap() {
        let cn_ds = field.content_disposition();
        let filename = cn_ds
            .get_filename()
            .map_or("".to_string(), |f| f.to_string());
        if filename == "".to_string() {
            return HttpResponse::BadRequest().body("filename is empty");
        }

        let mut f = String::new();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await.unwrap() {
            f.push_str(std::str::from_utf8(&chunk).unwrap());
        }

        let details = CACHE.get(&filename);

        if details.is_none() {
            return HttpResponse::BadRequest().body("descryption for file is empty");
        }

        let details = details.unwrap();

        SubActiveModel {
            path: Set(format!("./tmp/{}", filename)),
            language: Set(details.language),
            title: Set(details.title),
            description: Set(details.description),
            ..Default::default()
        }
        .save(&state.conn)
        .await
        .unwrap();

        CACHE.remove(&filename);
    }
    HttpResponse::Ok().body("Uploaded")
}

static CACHE: Lazy<MemDB> = Lazy::new(|| MemDB::new());

struct MemDB {
    data: Mutex<HashMap<String, UploadsDetails>>,
}

impl MemDB {
    fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }

    fn insert(&self, key: String, value: UploadsDetails) {
        self.data.lock().unwrap().insert(key, value);
    }

    fn get(&self, key: &str) -> Option<UploadsDetails> {
        self.data.lock().unwrap().get(key).cloned()
    }

    fn remove(&self, key: &str) {
        self.data.lock().unwrap().remove(key);
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct UploadsDetails {
    pub title: String,
    pub language: Option<String>,
    pub description: String,
}

async fn uploads_details(form: web::Form<UploadsDetails>) -> HttpResponse {
    let form = form.into_inner();
    CACHE.insert(
        form.title.clone(),
        UploadsDetails {
            title: form.title,
            language: form.language,
            description: form.description,
        },
    );

    HttpResponse::Ok().finish()
}
