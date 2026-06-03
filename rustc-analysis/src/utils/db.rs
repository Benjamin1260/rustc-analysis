use duckdb::Connection;


pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new() -> Self {
        let connection = Connection::open("placeholder.duckdb")
            .expect("failed to establish duckdb connection");

        connection.execute(Self::DB_SCHEME_SQL, ())
            .expect("failed to create tables");

        DB{conn: connection}
    }

    pub const DB_SCHEME_SQL: &'static str = "
        CREATE TABLE IF NOT EXISTS FieldList (
            field VARCHAR PRIMARY KEY
        );

        CREATE TABLE IF NOT EXISTS ProblemTree (
            problem VARCHAR PRIMARY KEY,
            parent VARCHAR,
            FOREIGN KEY (parent) REFERENCES ProblemTree(problem)
        );

        CREATE TABLE IF NOT EXISTS DefIdKinds (
            id UTINYINT PRIMARY KEY,
            name VARCHAR NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS Repos (
            id INTEGER PRIMARY KEY,
            repo_url VARCHAR NOT NULL UNIQUE,
            commit_hash VARCHAR NOT NULL
        );

        CREATE TABLE IF NOT EXISTS MergedCrates (
            id UUID PRIMARY KEY,
            name VARCHAR NOT NULL,
            src_url VARCHAR,
            field VARCHAR,
            FOREIGN KEY (field) REFERENCES FieldList(field)
        );

        CREATE TABLE IF NOT EXISTS Crates (
            id UUID PRIMARY KEY,
            src_repo INTEGER NOT NULL,
            name VARCHAR NOT NULL,
            version VARCHAR NOT NULL,
            internal BOOLEAN NOT NULL,
            path_url VARCHAR NOT NULL,
            merged_crate_id UUID,
            FOREIGN KEY (src_repo) REFERENCES Repos(id),
            FOREIGN KEY (merged_crate_id) REFERENCES MergedCrates(id),
            UNIQUE (src_repo, name, version, path_url)
        );

        CREATE TABLE IF NOT EXISTS DefIds (
            id UUID PRIMARY KEY,
            def_path_hash VARCHAR NOT NULL UNIQUE,
            crate_id UUID NOT NULL,
            def_path_str VARCHAR NOT NULL,
            kind UTINYINT NOT NULL,
            FOREIGN KEY (crate_id) REFERENCES Crates(id),
            FOREIGN KEY (kind) REFERENCES DefIdKinds(id)
        );

        CREATE TABLE IF NOT EXISTS Dependencies (
            from_def UUID NOT NULL,
            to_def UUID NOT NULL,
            PRIMARY KEY (from_def, to_def),
            FOREIGN KEY (from_def) REFERENCES DefIds(id),
            FOREIGN KEY (to_def) REFERENCES DefIds(id)
        );

        CREATE TABLE IF NOT EXISTS ManAnalysisResults (
            def_id UUID NOT NULL,
            problem VARCHAR NOT NULL,
            file_path VARCHAR NOT NULL,
            line_nr_start INTEGER NOT NULL,
            line_nr_end INTEGER NOT NULL,
            PRIMARY KEY (def_id, problem),
            FOREIGN KEY (def_id) REFERENCES DefIds(id),
            FOREIGN KEY (problem) REFERENCES ProblemTree(problem),
            CHECK (line_nr_start >= 0),
            CHECK (line_nr_end >= line_nr_start)
        );
    "; 
}