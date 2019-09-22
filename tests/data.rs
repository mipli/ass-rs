#[cfg(test)]
mod data_tests {
    use ::ass_rs::AssData;

    #[test]
    fn create_from_string() {
        let data: AssData = "{\"id\": 3, \"path\": \"path\"}"
            .parse()
            .expect("Could not parse from file");
        assert_eq!(data.get_id(), Some(3));
        assert_eq!(data.get_path(), Some("path"));
    }
}
