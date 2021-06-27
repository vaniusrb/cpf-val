#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;
#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::usize;

fn main() -> std::io::Result<()> {
    println!("Iniciando...");

    let file = File::open("res/cpfs.txt")?;
    let mut reader = BufReader::new(file);

    let mut cpf_buffer = [0i32; 11];
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

const POS_NUMS: [usize; 11] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13];

#[inline]
fn valida_cpf_str(cpfs_reg: &str, cpf_buffer: &mut [i32; 11]) -> bool {
    // Obtém números de posição fixa
    let bytes = cpfs_reg.as_bytes();
    for (i, b) in POS_NUMS.iter().enumerate() {
        cpf_buffer[i] = bytes[*b] as i32 - 48i32;
    }

    // Para considerar somente caracteres válidos (números)
    // let mut i = 0;
    // for c in cpfs_reg.as_bytes() {
    //     if (&48..=&57).contains(&c) {
    //         cpf_buffer[i] = *c as i32 - 48;
    //         i += 1;
    //     }
    // }

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

#[inline]
fn calcula_digitos(cpf_buffer: &[i32; 11]) -> (i32, i32) {
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    for i in 0..9 {
        sum_1 += cpf_buffer[i] * (10 - i as i32);
        sum_2 += cpf_buffer[i] * (11 - i as i32);
    }
    sum_2 += cpf_buffer[9] * 2;

    let mut remain = sum_1 % 11;
    let d1 = if remain > 1 { 11 - remain } else { 0 };

    remain = sum_2 % 11;
    let d2 = if remain > 1 { 11 - remain } else { 0 };

    (d1, d2)
}
