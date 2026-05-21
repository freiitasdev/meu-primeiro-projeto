fn main() {
    println!("Estimados formador João Canas e colegas da UFCD 10789, Boa tarde!");

    // Ciclo exterior
    for i in 1..=3 {
        println!("Ciclo exterior: {}", i);

        // Ciclo interior
        for j in 1..=5 {
            println!("   Ciclo interior: {}", j);
        }
    }
}