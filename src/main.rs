use std::{fs::File, io::Read, path::Path};

fn main() -> std::io::Result<()> {
    let path_messages = Path::new("messages.txt");
    let mut file = File::open(path_messages)?;
    let mut buffer = [0_u8; 8];
    //Ler arquivo de 8 em 8 bytes até o final
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n_bytes) => {
                let text = String::from_utf8_lossy(&buffer[..n_bytes]);
                print!("{text}");
            }
            Err(e) => {
                eprintln!("Erro ao ler arquivo: {e}");
                break;
            }
        }
    }
    Ok(())
}
