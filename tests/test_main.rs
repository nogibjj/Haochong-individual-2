#[cfg(test)]
mod tests {
    use Haochong_individual_2::extract;
    use Haochong_individual_2::create_table;
    use Haochong_individual_2::load_csv_into_db;
    use Haochong_individual_2::insert;
    use Haochong_individual_2::read;
    use Haochong_individual_2::update_shape_leng;
    use Haochong_individual_2::delete; 
    use rusqlite::Connection;

    const TEST_DB_PATH: &str = "test_db.db";
    const TEST_CSV_PATH: &str = "test_data.csv";

    #[test]
    fn test_extract_and_load_csv() {
        let url = "https://catalogue.data.wa.gov.au/dataset/f39087e2-2885-473e-bc62-ca610cd94340/resource/96c892f3-b387-410c-80d0-e4dcec68e6f2/download/25ktopomapseriesindex.csv";
        let csv_file_path = "25ktopomapseriesindex.csv";
        let db_path = "ktopomapseriesindexDB.db";

        // Simulate extracting data from a URL (you can use a mock HTTP server library)
        // This test does not actually download data from the internet
        // You can replace it with a real HTTP mocking library or a test data file.

        extract(url, csv_file_path).unwrap();
        
        let conn = Connection::open(db_path).unwrap();
        create_table(&conn).unwrap();

        // Load data from the CSV file into the database
        load_csv_into_db(csv_file_path).unwrap();

        // Assert that the database contains data
        let data = read(&conn).unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_insert_and_delete() {
        let conn = Connection::open(TEST_DB_PATH).unwrap();
        create_table(&conn).unwrap();

        let name_cap_2 = "Test Name";
        let num_rom_ca = "5678";
        let shape_leng = 345.67;
        let shape_area = 890.12;

        // Insert a record into the database
        insert(&conn, name_cap_2, num_rom_ca, shape_leng, shape_area).unwrap();

        // Read and assert that the inserted record is in the database
        let data = read(&conn).unwrap();
        assert_eq!(data.len(), 1);
        let record = &data[0];
        assert_eq!(record.1, name_cap_2);
        assert_eq!(record.2, num_rom_ca);
        assert_eq!(record.3, shape_leng);
        assert_eq!(record.4, shape_area);

        // Delete the record
        delete(&conn, num_rom_ca).unwrap();

        // Ensure that the record has been deleted
        let data = read(&conn).unwrap();
        assert!(data.is_empty());
    }
}


