use crate::model_views::admin_token::AdminToken;
use crate::models::admin::Admin;
use crate::services::jwt_service::generate_jwt_token;

pub fn get_admin_by_email_password(_email: String, _password: String) -> Option<Admin> {
    println!("{}", _email);
    println!("{}", _password);

    let found_admin = _email == "admin@example.com" && _password == "123456";

    if found_admin {
        Some(Admin {
            id: 1,
            name: "John Doe".to_string(),
            email: "admin@example.com".to_string(),
            password: "123456".to_string(),
        })
    } else {
        None
    }
}

pub fn login(email: String, password: String) -> Result<AdminToken, String> {
    let adm = get_admin_by_email_password(email, password);
    match adm {
        Some(adm) => Ok(
            AdminToken {
                id: adm.id as u32,
                name: adm.name,
                email: adm.email,
                token: generate_jwt_token(adm.id as u32),
            }
        ),
        None => Err("User not found".to_string()),
    }
}
