extern crate dotenv;

use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Database,
    sql::Thing,
    Surreal,
};

use dotenv::dotenv;
use std::{
    env,
    sync::{Arc, Mutex},
};

use std::time::Duration;
use tokio::time::sleep;

use agnus_daily_error::AgnusDailyError;
use tokio::sync::OnceCell;

pub struct DbClient {
    db: Arc<Mutex<Surreal<Client>>>,
}

pub mod tables_names;
pub mod queries;

impl DbClient {
    pub async fn new() -> Result<Self, AgnusDailyError> {
        let db_port =
            env::var("SURREAL_DB_PORT").expect("SURREAL_DB_PORT to exist in .env at data module");
        let db_address = format!("0.0.0.0:{}", db_port);
        let db = Surreal::new::<Ws>(db_address).await?;

        let db_user =
            env::var("SURREAL_DB_USER").expect("SURREAL_DB_USER to exist in .env at data module");
        let db_password = env::var("SURREAL_DB_PASSWORD")
            .expect("SURREAL_DB_PASSWORD to exist in .env at data module");
        let db_namespace =
            env::var("SURREAL_DB_NS").expect("SURREAL_DB_NS to exist in .env at data module");
        let db_database = env::var("SURREAL_DB_DATABASE")
            .expect("SURREAL_DB_DATABASE to exist in .env at data module");

        let credentials = Database {
            username: &db_user,
            namespace: &db_namespace,
            password: &db_password,
            database: &db_database,
        };

        db.signin(credentials).await?;

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }

    pub async fn select_many<R: for<'de> Deserialize<'de>>(
        &self,
        entity: &str,
    ) -> Result<Vec<R>, AgnusDailyError> {
        let lock = self.db.try_lock()?;
        Ok(lock.select(entity).await?)
    }

    pub async fn select_one<R: for<'de> Deserialize<'de>>(
        &self,
        entity_and_id: Thing,
    ) -> Result<Option<R>, AgnusDailyError> {
        let lock = self.db.try_lock()?;
        Ok(lock.select(entity_and_id).await?)
    }

    pub async fn create<R: for<'de> Deserialize<'de>, D: Serialize>(
        &self,
        entity_and_id: Thing,
        data: D,
    ) -> Result<Option<R>, AgnusDailyError> {
        let lock = self.db.try_lock()?;
        Ok(lock.create(entity_and_id).content(data).await?)
    }

    pub async fn query_vec<R: for<'de> Deserialize<'de>>(
        &self,
        query_str: &str,
    ) -> Result<Vec<R>, AgnusDailyError> {
        let lock = self.db.try_lock()?;
        let mut result = lock.query(query_str).await?;
        let result: Vec<R> = result.take(0)?;
        Ok(result)
    }

    pub async fn query_opt<R: for<'de> Deserialize<'de>>(
        &self,
        query_str: &str,
    ) -> Result<Option<R>, AgnusDailyError> {
        let lock = self.db.try_lock()?;
        let mut result = lock.query(query_str).await?;
        let result: Option<R> = result.take(0)?;
        Ok(result)
    }

    pub async fn query_b_opt<R: for<'de> Deserialize<'de>, B: Serialize>(
        &self,
        query_str: &str,
        bindings: B,
    ) -> Result<Option<R>, AgnusDailyError> {
        let lock = self.db.try_lock()?;
        let mut result = lock.query(query_str).bind(bindings).await?;
        let result: Option<R> = result.take(0)?;
        Ok(result)
    }

    pub async fn query_b_vec<R: for<'de> Deserialize<'de>, B: Serialize>(
        &self,
        query_str: &str,
        bindings: B,
    ) -> Result<Vec<R>, AgnusDailyError> {
        let lock = self.db.try_lock()?;
        let mut result = lock.query(query_str).bind(bindings).await?;
        let result: Vec<R> = result.take(0)?;
        Ok(result)
    }

    pub async fn update_content<R: for<'de> Deserialize<'de>, D: Serialize>(
        &self,
        entity_and_id: Thing,
        data: D,
    ) -> Result<Option<R>, AgnusDailyError> {
        let lock = self.db.try_lock()?;
        Ok(lock.update(entity_and_id).content(data).await?)
    }

    pub async fn update_merge<R: for<'de> Deserialize<'de>, D: Serialize>(
        &self,
        entity_and_id: Thing,
        data: D,
    ) -> Result<Option<R>, AgnusDailyError> {
        let lock = self.db.try_lock()?;
        Ok(lock.update(entity_and_id).merge(data).await?)
    }

    pub async fn update_patch<R: for<'de> Deserialize<'de>, D: Serialize>(
        &self,
        entity_and_id: Thing,
        data: D,
    ) -> Result<Option<R>, AgnusDailyError> {
        let lock = self.db.try_lock()?;
        Ok(lock.update(entity_and_id).merge(data).await?)
    }
}

static DB: OnceCell<DbClient> = OnceCell::const_new();

pub async fn db() -> &'static DbClient {
    DB.get_or_init(move || async {
        dotenv().ok();
        loop {
            let client = DbClient::new().await;
            if let Err(error) = client {
                println!("connection failed! {:?}", error);
                sleep(Duration::from_secs(1)).await;
                continue;
            };
            return client.unwrap();
        }
    })
    .await
}

// #[tokio::test]
// pub async fn insert_data() {
//     use enums::liturgy::{CommemorationCategory, LiturgicalColor, LiturgicalTime};
//     use structs::{
//         dtos::{
//             CreateCommemorationEntityDto, CreateCommemorationTitleEntityDto, CreateGospelEntityDto,
//             CreateLiturgicalDateEntityDto, CreateVideoAssetEntityDto,
//         },
//         entities::{
//             CommemorationEntity, CommemorationTitleEntity, GospelEntity, LiturgicalDateEntity,
//             VideoAssetEntity,
//         },
//     };
//     use surrealdb::sql::{Array, Id};

//     let db = db().await;

//     let gospel_uuid = String::from("Mt_5:27-32");

//     let create_commemoration_dto = CreateCommemorationEntityDto {
//         category: CommemorationCategory::Regular,
//         color: LiturgicalColor::Green,
//         gospel_uuid: gospel_uuid.clone(),
//     };

//     let commemoration_uuid = "ord_w10_ae_fri";
//     let commemoration_id: Thing = ("commemorations", commemoration_uuid).into();

//     let create_commemoration_result = db
//         .create::<CommemorationEntity, CreateCommemorationEntityDto>(
//             commemoration_id.clone(),
//             create_commemoration_dto,
//         )
//         .await;

//     println!(
//         "Create Commemoration result = {:?}",
//         create_commemoration_result
//     );

//     let day = 14;
//     let month = 6;
//     let year = 2024;

//     let create_liturgical_date_dto = CreateLiturgicalDateEntityDto {
//         time: LiturgicalTime::Ordinary,
//         day,
//         month,
//         year,
//         commemorations_ids: vec![commemoration_id],
//     };

//     let create_liturgical_date_result = db
//         .create::<LiturgicalDateEntity, CreateLiturgicalDateEntityDto>(
//             (
//                 "liturgical_dates",
//                 format!("{}-{}-{}", day, month, year).as_str(),
//             )
//                 .into(),
//             create_liturgical_date_dto,
//         )
//         .await;

//     println!(
//         "Create Liturgical Date Result = {:?}",
//         create_liturgical_date_result
//     );

//     let create_commemoration_title_en_dto = CreateCommemorationTitleEntityDto {
//         title: String::from(" Friday in the 10ᵗʰ Week in Ordinary Time"),
//     };

//     let create_commemoration_title_en_result = db
//         .create::<CommemorationTitleEntity, CreateCommemorationTitleEntityDto>(
//             Thing {
//                 tb: String::from("commemoration_titles"),
//                 id: Id::Array(Array(vec![
//                     String::from(commemoration_uuid).into(),
//                     String::from("en_US").into(),
//                 ])),
//             },
//             create_commemoration_title_en_dto,
//         )
//         .await;

//     println!(
//         "Create Commemoration Title en_US result = {:?}",
//         create_commemoration_title_en_result
//     );

//     let create_commemoration_title_pt_dto = CreateCommemorationTitleEntityDto {
//         title: String::from("Sexta-Feira da Semana X do Tempo Comum"),
//     };

//     let create_commemoration_title_pt_result = db
//         .create::<CommemorationTitleEntity, CreateCommemorationTitleEntityDto>(
//             Thing {
//                 tb: String::from("commemoration_titles"),
//                 id: Id::Array(Array(vec![
//                     String::from(commemoration_uuid).into(),
//                     String::from("pt_BR").into(),
//                 ])),
//             },
//             create_commemoration_title_pt_dto,
//         )
//         .await;

//     println!(
//         "Create Commemoration Title pt_BR result = {:?}",
//         create_commemoration_title_pt_result
//     );

//     let create_video_asset_dto = CreateVideoAssetEntityDto {
//         yt_id: String::from("FlAXKUp8-X0"),
//         thumbnail_url: None,
//     };

//     let video_asset_uuid = gospel_uuid.clone();

//     let video_asset_en_id = Thing {
//         tb: String::from("video_assets"),
//         id: Id::Array(Array(vec![
//             video_asset_uuid.clone().into(),
//             String::from("en_US").into(),
//         ])),
//     };

//     let create_video_asset_pt_result = db
//         .create::<VideoAssetEntity, CreateVideoAssetEntityDto>(
//             video_asset_en_id.clone(),
//             create_video_asset_dto.clone(),
//         )
//         .await;

//     println!(
//         "Create Video Asset en_US result = {:?}",
//         create_video_asset_pt_result
//     );

//     let video_asset_pt_id = Thing {
//         tb: String::from("video_assets"),
//         id: Id::Array(Array(vec![
//             video_asset_uuid.clone().into(),
//             String::from("pt_BR").into(),
//         ])),
//     };

//     let create_video_asset_pt_result = db
//         .create::<VideoAssetEntity, CreateVideoAssetEntityDto>(
//             video_asset_pt_id.clone(),
//             create_video_asset_dto,
//         )
//         .await;

//     println!(
//         "Create Video Asset pt_BR result = {:?}",
//         create_video_asset_pt_result
//     );

//     let video_assets_uuids = vec![video_asset_uuid];

//     let gospel_id = ("gospels", gospel_uuid.as_str()).into();
//     let create_gospel_dto = CreateGospelEntityDto { video_assets_uuids };

//     let create_gospel_result = db
//         .create::<GospelEntity, CreateGospelEntityDto>(gospel_id, create_gospel_dto)
//         .await;

//     println!("Create Gospel result = {:?}", create_gospel_result);

//     // let query = format!(
//     //     "SELECT * FROM dates WHERE (day = {} AND month = {} AND year = {})",
//     //     today_datetime.day(),
//     //     today_datetime.month(),
//     //     today_datetime.year()
//     // );

//     // let result = db.query_vec::<LiturgicalDateEntity>(&query).await;
// }

#[tokio::test]
pub async fn test_query() {
    use enums::liturgy::LiturgicalTime;
    use structs::entities::LiturgicalDateEntity;
    let db = db().await;

    // let query = format!("SELECT * FROM type::thing($id);");
    // SELECT * FROM liturgical_dates:⟨11-6-2024⟩

    let id: Thing = (
        "liturgical_dates",
        format!("{}-{}-{}", 11, 6, 2024).as_str(),
    )
        .into();

    let test = id.to_raw();
    let test2 = id.to_string();
    println!("RAW {}, STRING {}", test, test2);

    let query = format!("SELECT * FROM {};", id.to_string());

    // let result = db.select_one::<LiturgicalDateEntity>(id).await;

    let result = db.query_opt::<LiturgicalDateEntity>(&query).await;

    // let result = db
    //     .query_b_opt::<LiturgicalDateEntity, (&str, &str)>(&query, ("id", id.to_string().as_str()))
    //     .await;

    println!("THIS IS RESULT {:?}", result);
}

#[test]
fn conversion() {
    use std::mem;
    let number: u8 = 1;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    let number: u8 = 2;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    let number: u8 = 3;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    let number: u8 = 4;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    let number: u8 = 5;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    let number: u8 = 6;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    let number: u8 = 7;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    let number: u8 = 8;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    let number: u8 = 9;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    let number: u8 = 10;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    let number: u8 = 11;
    let bytes = number.to_ne_bytes();
    println!(
        "number{}, array: {:?}, size {}",
        number,
        bytes,
        mem::size_of_val(&bytes)
    );

    println!("Size of u8: {} bytes", mem::size_of::<u8>());
    println!("Size of i64: {} bytes", mem::size_of::<i64>());
    println!("Size of &str: {} bytes", mem::size_of::<&str>());
    println!("Size of &str: {} bytes", mem::size_of_val("WhiteOrGold"));
}
