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

    // Lista de developers portugueses fictícios
    println!("\n--- Lista de Developers ---");

    println!("Nome: Miguel Santos | Dev Rust | 5 anos de experiência");
    println!("Nome: Ricardo Ferreira | Dev C | 2 anos de experiência");
    println!("Nome: António Silva | Dev Assembly | 20 anos de experiência");

    println!("Nome: João Pereira | Dev Rust | 3 anos de experiência");
    println!("Nome: Tiago Oliveira | Dev Python | 4 anos de experiência");
    println!("Nome: Bruno Costa | Dev C++ | 7 anos de experiência");

    println!("Nome: Luís Almeida | Dev Java | 6 anos de experiência");
    println!("Nome: Pedro Martins | Dev Go | 2 anos de experiência");
    println!("Nome: Carlos Sousa | Dev Assembly | 15 anos de experiência");

    println!("Nome: André Rodrigues | Dev Rust | 1 ano de experiência");
    println!("Nome: Fábio Gomes | Dev C# | 8 anos de experiência");
    println!("Nome: Nuno Correia | Dev Embedded C | 12 anos de experiência");
}