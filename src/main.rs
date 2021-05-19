use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    println!("Iniciando...");

    let file = File::open("res/cpfs.txt")?;
    let mut reader = BufReader::new(file);

    let mut cpf_buffer = [0u32; 11];
    let mut validos = 0;
    let mut invalidos = 0;

    let mut line_buffer = String::new();

    while reader.read_line(&mut line_buffer)? > 0 {
        if valida_cpf_str(&line_buffer, &mut cpf_buffer) {
            validos += 1;
        } else {
            invalidos += 1;
        }
        line_buffer.clear();
    }

    println!("Terminado. Válidos {} inválidos {}", validos, invalidos);
    Ok(())
}

fn valida_cpf_str(cpfs_reg: &str, cpf_buffer: &mut [u32; 11]) -> bool {
    let mut i = 0;
    for c in cpfs_reg.chars() {
        if let Some(d) = c.to_digit(10) {
            cpf_buffer[i] = d;
            i += 1;
        }
    }
    let (d1, d2) = calcula_digitos(&cpf_buffer);
    if cpf_buffer[9] != d1 {
        println!("primeiro inválido {}", d1);
        false
    } else if cpf_buffer[10] != d2 {
        println!("segundo inválido {}", d2);
        false
    } else {
        true
    }
}

fn calcula_digitos(cpf_buffer: &[u32; 11]) -> (u32, u32) {
    let mut sum = 0;
    for i in 0..9 {
        sum += cpf_buffer[i] * (10 - i as u32)
    }
    let mut remain = sum % 11;
    let d1 = if remain > 1 { 11 - remain } else { 0 };

    sum = 0;
    for i in 0..10 {
        sum += cpf_buffer[i] * (11 - i as u32)
    }
    remain = sum % 11;
    let d2 = if remain > 1 { 11 - remain } else { 0 };

    (d1, d2)
}
