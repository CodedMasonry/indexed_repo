fn main() {
    match indexed_repo::tui() {
        Ok(_) => (),
        Err(e) => eprintln!("[-] Error: {:#?}", e),
    }
}
