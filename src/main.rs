use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(std::time::Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("ROBINSON JUMA MUSEMBI", 0, 1),
        Philosopher::new("ILYAS ABDI", 1, 2),
        Philosopher::new("KIMANI ABRAHAM KURIA", 2, 3),
        Philosopher::new("OCHIENG ANDY OMONDI", 3, 4),
        Philosopher::new("WAIYEGO ANTONY MWANGI", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}

/* 
KINDLY NOTE THAT THERE'S A PROGRAM I CREATED 9 MONTHS AGO BUT I REALISED THAT I DIDN'T DO ANYTHING THAT I WANTED TO DO SO I RENAMED AND MODIFIED IT TO SOLVE THE DINING PHILOSOPHER
That's why the progam is indicated that it was created 9months ago 
*/