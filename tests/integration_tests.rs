#[cfg(test)]
mod integration_tests {
    use pretty_assertions::{assert_eq, assert_ne};
    use std::path;

    fn testdata() -> path::PathBuf {
        path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("testdata")
    }

    fn test_file(filename: &str) -> path::PathBuf {
        testdata().join(filename)
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
        assert_ne!(file.data.len(), 0, "no survey data found");
        assert_eq!(
            file.data[0].explorer.as_ref().unwrap(),
            "<Explorer> Mr, Miyagi</Explorer><Surveyor>Bæ & Bu <oo> </Surveyor>"
        );
    }

    #[test]
    #[ignore = "Some files have missing geoCoding tags, some have empty, why?"]
    pub fn there_and_back() {
        let files = std::fs::read_dir(testdata())
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_name().to_str().unwrap().ends_with(".tmlu"));
        for file in files {
            println!("Parsing file: {:?}", file.path());
            let reader = std::fs::File::open(file.path()).unwrap();
            let size = reader.metadata().unwrap().len() as usize;
            let buf_reader = std::io::BufReader::new(reader);
            let cave = tmlu_rs::tmlu::read_cavefile(buf_reader);
            let mut output = std::io::Cursor::new(Vec::with_capacity(size));
            tmlu_rs::tmlu::write_cavefile(&mut output, cave.data, cave.info).unwrap();
            //compare the output with the original file
            let original = String::from_utf8(std::fs::read(file.path()).unwrap()).unwrap();
            let output = String::from_utf8(output.into_inner()).unwrap();
            assert_eq!(original, output);
        }
    }
}
