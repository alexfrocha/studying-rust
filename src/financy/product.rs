pub struct Produto {
    pub nome: String,
    pub preco: f32,
    pub quantidade: i32,
}

impl Produto {
    pub fn mudar_nome(&mut self, new_name: &str) {
        self.nome = new_name.to_string()
    }

    pub fn mudar_preco(&mut self, new_price: f32) {
        self.preco = new_price
    }

    pub fn mudar_quantidade(&mut self, new_quantity: i32) {
        self.quantidade = new_quantity
    }

    pub fn show(&self) {
        println!("Nome: {}", self.nome);
        println!("Preço: {}", self.preco);
        println!("Estoque: {}", self.quantidade);
    }
}