pub fn tem_caracteres_unicos(input: &str) -> bool {
    let mut conjunto_de_caracteres = [false; 128];
    for &c in input.as_bytes() {
        let indice = c as usize;
        println!("Caractere: {}, Índice: {}", c as char, indice);
        
        if conjunto_de_caracteres[indice] {
            return false
        }
        conjunto_de_caracteres[indice] = true;
    }
    true
}