pub fn main() {
    let mut coffre = 100;
    let carte = &mut coffre;

    *carte = 200;
    modifier_coffre(carte);

    println!("Carte:: {}", carte);
    println!("coffre mutable:: {}", coffre);
}

fn modifier_coffre(content: &mut i32) {
    *content += 100;
}
