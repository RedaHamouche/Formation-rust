use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};

fn main() {
    // Alice génère sa paire
    let alice_secret = EphemeralSecret::new(OsRng);
    let alice_public = PublicKey::from(&alice_secret);

    // Bob génère sa paire
    let bob_secret = EphemeralSecret::new(OsRng);
    let bob_public = PublicKey::from(&bob_secret);

    // Chacun calcule le secret partagé
    let alice_shared = alice_secret.diffie_hellman(&bob_public);
    let bob_shared = bob_secret.diffie_hellman(&alice_public);

    println!("Secret d’Alice : {:?}", alice_shared.as_bytes());
    println!("Secret de Bob   : {:?}", bob_shared.as_bytes());
    assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());
    println!("✅ Même secret !");
}
