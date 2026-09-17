use std::{fs::File, io::BufRead, io::BufReader, path::Path};

fn main() -> std::io::Result<()> {
    let path_messages = Path::new("messages.txt");
    let file = File::open(path_messages)?;
    let mut buffer = BufReader::new(file);
    //Ler arquivo de linha em linha
    let mut line = String::new();
    loop {
        line.clear();
        match buffer.read_line(&mut line) {
            Ok(0) => break,
            Ok(_n_bytes) => {
                print!("{line}");
            }
            Err(e) => {
                eprintln!("Erro ao ler arquivo: {e}");
                break;
            }
        }
    }
    Ok(())
}
