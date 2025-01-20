struct DatabaseConnection {
    id: i32,
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("Close db connection {}", self.id);
    }
}

fn main() {
    println!("program start");
    {
        let conn1 = DatabaseConnection { id: 1 };
        {
            let conn2 = DatabaseConnection { id: 2 };
            println!("Open Connection");
        } // conn2 is dropped here
        println!("Inner block finish");
    } // conn1 is dropped here

    println!("program finish");
}
