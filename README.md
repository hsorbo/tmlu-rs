# tmlu-rs

A rust library for reading and writing tmlu-files used for cave survey.
It's currently a work-in-progress.

Goals:

- Support all datatypes
- Fast
- Low memory footprint
- Outputs binary same output as original (for keeping VCS history clean)
  
Non goals:

- DOM

## Running or installing the example apps

```bash
# Running
cargo run --release --example tmlu2json ~/megacave.tmlu > megacave.json

# Installing
cargo install --path . --example tmlu2json
cargo install --path . --example tmlu2sqlite

# Running installed
tmlu2json ~/megacave.tmlu > megacave.json
tmlu2sqlite tmlu-to-sqlite --tmlu ~/megacave.tmlu --database megacave.sqlite

```

## SQLite queries to try

```bash
sqlite3 megacave.sqlite
```

```sql

-- Average depth of stations
select avg(depth) from survey_data where excluded = 'false';

-- Exploration length per year
select 
  sum(length), 
  strftime("%Y", date) as year 
from 
  survey_data 
group by 
  year;

-- Get gps-cooridnates of all start-stations
select latitude || "," || longitude from survey_data where station_type = 'START';
```

You can also store output in json for further analysis

```bash
sqlite3 -json megacave.sqlite 'select id, comment from survey_data where comment is not null' > megacave.json
```

Pro tip: The SQLite create table (in tmlu2sqlite.rs) can be fed to chatgpt to help creating queries.
