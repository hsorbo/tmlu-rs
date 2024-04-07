#[cfg(test)]
mod integration_tests {
    use std::path;

    fn test_file(filename: &str) -> path::PathBuf {
        path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("testdata")
            .join(filename)
    }

    fn open_test_file(filename: &str) -> std::io::BufReader<std::fs::File> {
        let testfile = test_file(filename);
        let reader = std::fs::File::open(testfile).unwrap();
        std::io::BufReader::new(reader)
    }

    #[test]
    pub fn parse_comments() {
        let testfile = open_test_file("test1.tmlu");
        let file = tmlu_rs::tmlu::read_cavefile(testfile);
        assert_eq!(
            file.data[0].explorer.as_ref().unwrap(),
            "<Explorer> Mr, Miyagi</Explorer><Surveyor>Bæ & Bu <oo> </Surveyor>"
        );
    }
}
