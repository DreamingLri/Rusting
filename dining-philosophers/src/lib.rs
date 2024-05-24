use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        println!("{} is trying to eat", &self.name);
        let left = self.left_fork.lock().unwrap();
        let right = self.right_fork.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {

    let (tx, rx) = mpsc::sync_channel(10);

    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();
    // Create forks
    for i in 0..forks.len(){
        let tx = tx.clone();
        let mut left_fork = Arc::clone(&forks[i]);
        let mut right_fork = Arc::clone(&forks[(i+i) % forks.len()]);

        // fix deadlock
        if i == forks.len() - 1{
            std::mem::swap(&mut left_fork, &mut right_fork);
        }
        // Create philosophers
        let philosopher = Philosopher{
            name: PHILOSOPHERS[i].to_string(),
            left_fork,
            right_fork,
            thoughts: tx
        };
        // Make each of them think and eat 100 times
        thread::spawn(move || {
            for _ in 0..100{
                philosopher.eat();
                philosopher.think();
            }
        });
    }
    drop(tx);
    // Output their thoughts
    for thoughts in rx{
        println!("{thoughts}");
    }

}