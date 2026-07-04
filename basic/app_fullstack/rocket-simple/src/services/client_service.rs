use crate::models::client::Client;

pub fn get_clients() -> Vec<Client> {
    vec![
        Client { id: 1, name: "John Doe".to_string(), cpf: "123.456.789-00".to_string() },
        Client { id: 2, name: "Jane Smith".to_string(), cpf: "987.654.321-00".to_string() },
        Client { id: 3, name: "Alice Johnson".to_string(), cpf: "456.789.123-00".to_string() },
        Client { id: 4, name: "Bob Brown".to_string(), cpf: "321.654.987-00".to_string() },
        Client { id: 5, name: "Charlie Davis".to_string(), cpf: "654.321.987-00".to_string() },
    ]
}

pub fn create_client(name:String, cpf:String) -> bool {
    // salvar os dados no banco de dados
    println!("Client Name {}", name);
    println!("Client Cpf {}", cpf);
    true // false
}

pub fn update(id:u32, name:String, cpf:String) -> bool {
    // salvar os dados no banco de dados
    println!("Client Id {}", id);
    println!("Client Name {}", name);
    println!("Client Cpf {}", cpf);
    true
}


pub fn delete_by_id(id:u32) -> bool {
    // simula exclusão de dados
    println!("Client Id {}", id);
    true
}

pub fn get_client_by_id(_id: u32) -> Client {
    // Simulação de retornando um dado no banco de dados de cliente
    println!("Client Id {}", _id);
    Client { id: _id, name: "Client 1".to_string(), cpf: "000.000.000-01".to_string() }
}
