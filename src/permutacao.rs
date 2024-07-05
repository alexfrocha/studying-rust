pub fn is_permutation(str: &str, str2: &str) -> bool {
    if str.len() != str2.len() {
        return false;
    }
    let mut contagem_caracteres = [0; 128]; // caracteres ascii
    for &c in str.as_bytes() {
        contagem_caracteres[c as usize] += 1;

    }

    for &c in str2.as_bytes() {
        contagem_caracteres[c as usize] -= 1;
        if contagem_caracteres[c as usize] < 0 {
            return false;
        }
    }

    true
}