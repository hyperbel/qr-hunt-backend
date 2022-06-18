use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
    

fn main() -> Result<()> {
    let conn = Connection::open("qr.db")?;

    conn.execute(
        "create table if not exists qr_code (
             id integer,
             userid integer,
             hash text not null,

         );
         create table if not exists user (
            rowid
             namen text not null unique,
             pwhash text not null unique,
         );
         create table if not exists qr_lookup (
             id integer primary key,
             content text not null unique,
             hash text not null unique,
             kordinaten text not null unique
         );",
        NO_PARAMS,
    )?;

    Ok(())
}