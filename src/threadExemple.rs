use std::thread;
use std::sync::mpsc;
fn main() {
// on crée le channel
let (tx, rx) = mpsc::channel();
for _ in 0..10 {
let tx = tx.clone();
thread::spawn(move || {
let answer = 42u32;
// on envoie la donnée dans le channel
tx.send(answer);
});
}
match rx.recv() {
Ok(data) => println!("Le channel vient de recevoir : {}", data),
Err(e) => println!("Une erreur s'est produite : {:?}", e)
};
}