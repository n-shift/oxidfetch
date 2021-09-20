pub fn fetch() -> Vec<String> {
    vec![whoami::username(), whoami::hostname()]
}
