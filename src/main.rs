use taskchampion::{storage::{SqliteStorage, Storage}, Replica};

fn main() {
    let db_path = ".";
    let mut app = App::new(db_path);

    println!("{:?}", app.replica.all_tasks());
}

struct App {
    replica: Replica,
}

impl App {

    pub fn new(db_path: &str) -> App {
        let sqlstorage = SqliteStorage::new(db_path, false).expect("failed to load database");
        let replica = Replica::new(Box::new(sqlstorage));
        App {
            replica
        }
    }   
}
