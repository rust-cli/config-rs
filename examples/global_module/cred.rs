use super::settings::Settings;

pub fn list_cred() {
    match Settings::user() {
        Ok(u) => println!("My name is: {u}"),
        Err(e) => println!("{e}")
    }

    match Settings::key() {
        Ok(k) => println!("My key is: {k}"),
        Err(e) => println!("{e}")
    }
}
