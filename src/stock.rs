fn lucro_maximo(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    
    let mut preco_minimo = prices[0];
    let mut lucro_maximo = 0;

    for preco in prices.iter().skip(1) {
        let lucro_atual = preco - preco_minimo;
        lucro_maximo = lucro_maximo.max(lucro_atual);
        preco_minimo = preco_minimo.min(*preco);
        
    }

    lucro_maximo
}
