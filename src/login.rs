pub struct Login {
    pub email: &'static str,
    pub password: &'static str,
}

pub fn login() -> Login {
    Login {
        email: "", // to be filled in
        password: "", // to be filled in
    }
}