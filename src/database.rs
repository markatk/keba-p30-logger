use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::wallbox::Wallbox;

pub struct Database {
    connection: PgConnection
}

impl Database {
    pub fn new() -> Result<Database, ()> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let connection = PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url));

        Ok(Database {
            connection
        })
    }

    pub fn add_wallbox_data(&mut self, wallbox: &Wallbox) {
        let data: crate::models::NewWallboxData = wallbox.into();

        diesel::insert_into(crate::schema::wallbox_data::table)
            .values(data)
            .execute(&self.connection)
            .expect("Error saving new wallbox data");
    }
}
