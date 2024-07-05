

#[derive(Debug)]
enum GeneroType {
    Male,
    Female
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    password: String,
    cpf: String,
    active: bool,
    genero: GeneroType
}

impl User {
    fn nome_do_usuario(&self) {
        println!("o nome do usuário é {}", &self.username);
    }
    fn ativo_ou_inativo(&self) {
        println!("o usuário está ativo? {}", &self.active);
    }
}

fn main() {
    let mut pessoa = User {username:String::from("alexfrocha"), email:String::from("alex.raskav@gmail.com"), password:String::from("123"), cpf:String::from("123"), active: true, genero: GeneroType::Male};
    pessoa.username = String::from("alexraskav");
    pessoa.nome_do_usuario();
    pessoa.ativo_ou_inativo();
}