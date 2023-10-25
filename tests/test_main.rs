#[cfg(test)]
mod tests {
    use Haochong_individual_2::extract;
    use Haochong_individual_2::create_table;
    use Haochong_individual_2::load_transform;
    use Haochong_individual_2::insert;
    use Haochong_individual_2::read;
    use Haochong_individual_2::update_shape_leng;
    use Haochong_individual_2::delete; 
    use rusqlite::Connection;

    const TEST_DB_PATH: &str = "ktopomapseriesindexDB.db";
    const TEST_CSV_PATH: &str = "25ktopomapseriesindex.csv";

    #[test]
    fn test_extract_and_load() {
        let url = "https://catalogue.data.wa.gov.au/dataset/f39087e2-2885-473e-bc62-ca610cd94340/resource/96c892f3-b387-410c-80d0-e4dcec68e6f2/download/25ktopomapseriesindex.csv";
        let csv_file_path = "25ktopomapseriesindex.csv";
        let db_path = "ktopomapseriesindexDB.db";

        extract(url, csv_file_path).unwrap();
        
        let conn = Connection::open(db_path).unwrap();
        create_table(&conn).unwrap();

        // Load data from the CSV file into the database
        load_transform(csv_file_path).unwrap();

        // Assert that the database contains data
        let data = read(&conn).unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_update_shape_leng() {
        let conn = Connection::open(TEST_DB_PATH).unwrap();
        create_table(&conn).unwrap();

        // Create a test record in the database
        insert(&conn, "Test Update", "Test UNum", 1.1, 2.2).unwrap();

        // Update the shape length
        let new_shape_leng = 3.3;
        let num_rom_ca_to_update = "Test UNum";
        update_shape_leng(&conn, new_shape_leng, num_rom_ca_to_update).unwrap();

        // Fetch the updated record
        let updated_record = read(&conn).unwrap();

        let (id, name_cap_2, num_rom_ca, shape_leng, shape_area) = &updated_record[updated_record.len()-1];

        // Check if the shape length was updated
        assert_eq!(num_rom_ca, "Test UNum");
        assert_eq!(shape_leng, &new_shape_leng);
    }

    #[test]
    fn test_insert_and_delete() {
        let conn = Connection::open(TEST_DB_PATH).unwrap();
        create_table(&conn).unwrap();

        let name_cap_2 = "Test Delete";
        let num_rom_ca = "Test DNum";
        let shape_leng = 5.5;
        let shape_area = 6.6;

        // Insert a record into the database
        insert(&conn, name_cap_2, num_rom_ca, shape_leng, shape_area).unwrap();

        // Read and assert that the inserted record is in the database
        let data = read(&conn).unwrap();
        let record = &data[data.len()-1];
        assert_eq!(record.1, name_cap_2);
        assert_eq!(record.2, num_rom_ca);
        assert_eq!(record.3, shape_leng);
        assert_eq!(record.4, shape_area);

        // Delete the record
        delete(&conn, num_rom_ca).unwrap();

        // Ensure that the record has been deleted
        let data = read(&conn).unwrap();
        let record = &data[data.len()-1];
        assert_ne!(record.1, name_cap_2);
        assert_ne!(record.2, num_rom_ca);
        assert_ne!(record.3, shape_leng);
        assert_ne!(record.4, shape_area);
    }
}


