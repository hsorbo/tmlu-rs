use clap::{command, Args, Parser, Subcommand};
use std::io::{BufReader, BufWriter};
use tmlu_rs::tmlu::{read_cavefile, write_cavefile, CaveFileInfo};

mod db {
    use rusqlite::{params, Connection};
    use tmlu_rs::tmlu::{Shape, SurveyData};

    pub fn get_survey_data(db: &str) -> Result<Vec<SurveyData>, rusqlite::Error> {
        let conn = Connection::open(db)?;
        let sel = "SELECT id,
            cast(azimuth as TEXT) as azimuth,
            closure_to_id,
            color,
            comment,
            date,
            depth,
            depth_in,
            down,
            excluded,
            explorer,
            from_id,
            inclination,
            latitude,
            left,
            length,
            locked,
            longitude,
            name,
            profile_type,
            cast(right as TEXT) as right,
            section,
            station_type,
            up from survey_data;";
        let mut stmt = conn.prepare(sel)?;
        let mut data = Vec::new();
        let rows = stmt.query_map([], |row| {
            let record = SurveyData {
                id: row.get(0)?,
                azimuth: row.get(1)?,
                closure_to_id: row.get(2)?,
                color: row.get(3)?,
                comment: row.get(4)?,
                date: row.get(5)?,
                depth: row.get(6)?,
                depth_in: row.get(7)?,
                down: row.get(8)?,
                excluded: row.get(9)?,
                explorer: row.get(10)?,
                from_id: row.get(11)?,
                inclination: row.get(12)?,
                latitude: row.get(13)?,
                left: row.get(14)?,
                length: row.get(15)?,
                locked: row.get(16)?,
                longitude: row.get(17)?,
                name: row.get(18)?,
                profile_type: row.get(19)?,
                right: row.get(20)?,
                section: row.get(21)?,
                shape: Shape::default(),
                //SH: row.get(22)?,
                station_type: row.get(22)?,
                up: row.get(23)?,
            };
            Ok(record)
        })?;
        for name_result in rows {
            data.push(name_result?);
        }
        Ok(data)
    }

    const CREATE_SRVD  : &str = "
        CREATE TABLE survey_data(
            id             INTEGER PRIMARY KEY,
            azimuth        TEXT,      -- Azimuth, bearing of a station (should be REAL)
            closure_to_id  INTEGER,   -- Closure to id, (informs that from_id and closure_to_id is same station) only relevent when station_type is 'CLOSURE'
            color          TEXT,      -- Color of line to station in hex ARGB (example 0xccffccff)
            comment        TEXT NULL, -- Comment, string or NULL
            date           TEXT,      -- Date when station was surveyed, in format YYYY-MM-DD
            depth          TEXT,      -- Depth of station (should be REAL)
            depth_in       TEXT,      -- Unknown
            down           TEXT,      -- Unknown
            excluded       TEXT,      -- Excluded, toggles visibility in UI
            explorer       TEXT,      -- Explorers / Surveyors
            from_id        INTEGER,   -- From station ID, -1 if none, applies when station_type is not 'START'
            inclination    TEXT,      -- Unknown
            latitude       TEXT,      -- Latitude, only relevant when station_type is 'START' (too long for REAL)
            left           TEXT,      -- Unknown
            length         TEXT,      -- Length of line from previous station to this one, not relevant when TY is 'START'
            locked         TEXT,      -- Unknown, boolean
            longitude      TEXT,      -- Longitude, only relevant when station_type is 'START' (too long for REAL)
            name           TEXT NULL, -- Name of station
            profile_type   TEXT,      -- Unknown 'VERTICAL', 'HORIZONTAL'
            right          TEXT,      -- Unknown
            section        TEXT NULL, -- Section name
            -- SH 
            station_type   TEXT,      -- Type of station, one of 'REAL', 'VIRTUAL', 'START', 'CLOSURE'
            up             TEXT);
        ";

    pub fn create_db(db: &str, data: Vec<SurveyData>) -> Result<(), rusqlite::Error> {
        let conn = Connection::open(db)?;
        conn.execute("BEGIN TRANSACTION;", [])?;
        conn.execute(CREATE_SRVD, [])?;
        for d in data {
            conn.execute(
            "INSERT INTO survey_data (id, azimuth, closure_to_id, color, comment, date, depth, depth_in, down, excluded, explorer, from_id, inclination, latitude, left, length, locked, longitude, name, profile_type, right, section, station_type, up)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24);",
             params![&d.id, &d.azimuth, &d.closure_to_id, &d.color, &d.comment, &d.date, &d.depth, &d.depth_in, &d.down, &d.excluded, &d.explorer, &d.from_id, &d.inclination, &d.latitude, &d.left, &d.length, &d.locked, &d.longitude, &d.name, &d.profile_type, &d.right, &d.section, &d.station_type, &d.up])?;
        }
        conn.execute("COMMIT;", [])?;
        Ok(())
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct Sqlite {
    /// sqlite3 database filename
    #[arg(short, long)]
    database: String,

    /// tmlu filename
    #[arg(short, long)]
    tmlu: String,
}

#[derive(Subcommand)]
enum Commands {
    SqliteToTmlu(Sqlite),
    TmluToSqlite(Sqlite),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::SqliteToTmlu(args) => {
            let srvd = db::get_survey_data(&args.database)?;
            let output = BufWriter::new(std::fs::File::create(&args.tmlu)?);
            write_cavefile(output, srvd, CaveFileInfo::default())?;
        }
        Commands::TmluToSqlite(args) => {
            let input = BufReader::new(std::fs::File::open(&args.tmlu)?);
            let cavefile = read_cavefile(input);
            db::create_db(&args.database, cavefile.data)?;
        }
    }
    Ok(())
}
