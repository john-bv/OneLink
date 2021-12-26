use onelink_database::db;

#[test]
pub fn test_open_db() {
    let db = Database::open("./tests/db/test.db").unwrap();
    assert_eq!(db.get_path(), "./tests/db/test.db");
}