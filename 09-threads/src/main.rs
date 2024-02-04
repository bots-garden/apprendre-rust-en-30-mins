use std::thread;
use std::time::Duration;

fn main() {
    // Créer un vecteur pour stocker les threads
    let mut handles = vec![];

    for i in 0..5 {
        // Créer un nouveau thread pour chaque itération
        let handle = thread::spawn(move || {
            // Code exécuté dans chaque thread
            for j in 0..3 {
                println!("Thread {}: Iteration {}", i, j);
                thread::sleep(Duration::from_millis(100)); // Simuler une tâche prenant du temps
            }
        });

        // Ajouter le thread au vecteur
        handles.push(handle);
    }

    // Attendre que tous les threads se terminent
    for handle in handles {
        handle.join().unwrap();
    }
}