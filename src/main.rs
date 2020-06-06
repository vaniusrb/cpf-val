fn main() -> std::io::Result<()> {
    println!("Iniciando...");

    let mut reader = my_reader::BufReader::open("res/cpfs.txt")?;

    let mut validos = 0;
    let mut invalidos = 0;

    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        if valida_cpf_str(line?) {
            validos += 1;
        } else {
            invalidos += 1;
        }
    }
    println!("Terminado. Válidos {} inválidos {}", validos, invalidos);
    Ok(())
}

fn valida_cpf_str(cpfs_reg: &str) -> bool {
    let mut cpf_num = [0i32; 11];
    let mut i = 0;
    for c in cpfs_reg.chars() {
        if let Some(d) = c.to_digit(10) {
            cpf_num[i] = d as i32;
            i += 1;
        }
    };
    valida_cpf(&cpf_num)
}

fn valida_cpf(cpf_num: &[i32; 11]) -> bool {
    let (d1, d2) = calcula_digitos(&cpf_num);
    if cpf_num[9] != d1 {
        print!("primeiro inválido {}\n", d1);
        false
    } else if cpf_num[10] != d2 {
        print!("segundo inválido {}\n", d2);
        false
    } else {
        true
    }
}

fn calcula_digitos(cpf_num: &[i32; 11]) -> (i32, i32) {
    let mut summ = 0;
    for i in 0..9 {
        summ += cpf_num[i] * (10 - i as i32)
    }
    let mut remain = summ % 11;
    let d1 = if remain > 1 { 11 - remain } else { 0 };

    summ = 0;
    for i in 0..10 {
        summ += cpf_num[i] * (11 - i as i32)
    }
    remain = summ % 11;
    let d2 = if remain > 1 { 11 - remain } else { 0 };

    (d1, d2)
}

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}
