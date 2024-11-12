use std::io;

fn main() {
    println!("Hello, Rust Programmer!");
    println!("Masukan Nama Anda : ");

    // variabel mutable -> variabel yang boleh di rubah nilainya
    // aslinya semua variabel hukumnya const
    let mut nama = String::new();
    let mut value1 = String::new();

    //ambil data dari terminial dengan  io::stdin.read_line
    io::stdin().read_line(&mut nama).expect("gagal");

    //tampilkan hasil inputan setelah masuk ke dalam fn
    println!("{}", greeting(nama.trim()));

    //membaca inputan angka
    println!("Masukkan Angka untuk di pangkatkan");
    io::stdin().read_line(&mut value1).expect("gagal");

    //ubah menjadi bilangan, karena default .read_line adalah string
    let value1 :i64 = value1.trim().parse().expect("harus angka");

    //operasi perkalian sederhana
    let result = value1 * value1;
    let result2 = result * result;
    println!(
        "hasil value pangkat 2 : {0}, \nsedangkan value pangkat 2 dipangkat 2 : {1}\n",
        result, result2
    );

    //pengkondisian
    if result2 >= 1000 {
        println!("hasil lebih dari ambang batas, nilainya {}", result2);
    } else {
        println!("hasil = {}", result2);
    }
}

fn greeting(salam: &str) -> String {
    format!("Haii .. , selamat datang {}", salam)
}
