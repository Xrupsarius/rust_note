pub mod sqlite_utils {
    use sqlite::Connection;

    static BD_MEMORY_NAME: &'static str = ":memory:";

    pub fn create_database(name: String) {
        sqlite::open(name).unwrap();
    }

    pub fn create_table(t_name: String, bd_name: String) {
        let connection = get_bd(bd_name);
        let query = format!("CREATE TABLE IF NOT EXISTS {t_name} (name TEXT);");
        connection.execute(query).unwrap();
    }

    fn get_bd(bd_name: String) -> Connection {
        let connection;
        if !bd_name.is_empty() {
            connection = sqlite::open(bd_name).unwrap();
        } else {
            connection = sqlite::open(BD_MEMORY_NAME).unwrap();
        }
        connection
    }

    pub fn remove_table(t_name: String, bd_name: String) {
        let connection = get_bd(bd_name);
        let query = format!("DROP TABLE IF EXISTS {t_name};");
        connection.execute(query).unwrap();
    }
}

