use financy::{blockchain::Block, coin::Coin, portfolio::Asset, product::Produto, BankAccount::{Account, Currency}};

pub mod financy;


fn main() {
    let portfolio: [Asset; 4] = [Asset::Stocks, Asset::Bond, Asset::Funds, Asset::Cash];
    let total: f64 = portfolio.iter().map(|asset: &Asset| asset.price()).sum();
    println!("E o valor total do seu portfólio é: {}", total);

    let mut coin = Coin::new(10);
    println!("O valor da moeda é de {} reais.", coin.get_value());
    coin.set_value(32);
    println!("O valor da moeda agora é de {} reais.", coin.get_value());

    let mut meu_produto = Produto{
        nome: "Produto".to_string(),
        preco: 10.99,
        quantidade: 1
    };

    meu_produto.mudar_nome("josébinho");
    meu_produto.show();

    let my_block = Block::new(0, 1823812838, "dados do bloco".to_string(), "hash".to_string(), "hash anterior".to_string());
    let size = my_block.data_size();
    let time = my_block.creation_time();
    println!("O tamanho do bloco é {} e tempo de criação é {} segundos.", size, time);

    let mut account = Account::new(1000.0, Currency::BRL);
    account.check_balance();
    account.convert_to(Currency::USD);
    account.check_balance();
    account.deposit(300000 as f64);
    account.check_balance();
    if account.withdraw(5000 as f64) {
        println!("Um valor de 5000 USD foi sacado!")
    } else {
        println!("Você não saldo suficiente")
    }
    account.check_balance();
    account.convert_to(Currency::EUR);
    account.check_balance();
}
