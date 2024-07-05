
pub fn calcula_media(notas: &[f32]) -> f32 {
    let tamanho = notas.len();
    let mut soma = 0.0;
    for nota in notas {
        soma += *nota;
    }
    return soma / tamanho as f32;
}