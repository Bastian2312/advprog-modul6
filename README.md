## Comit 1
Berikut implementasi fungsi handle_connection pada kode:

```
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader 
        .lines() 
        .map(|result| result.unwrap()) 
        .take_while(|line| !line.is_empty()) 
        .collect();
    println!("Request: {:#?}", http_request);
}
```

Fungsi ini menerima parameter bertipe TcpStream yang merepresentasikan koneksi TCP antara server dan klien. Kata kunci mut pada parameter menunjukkan bahwa stream bersifat mutable (dapat diubah), karena operasi pembacaan data dari stream akan mengubah status internalnya.

Inti dari fungsi ini terletak pada penggunaan BufReader yang dibangun dari TcpStream. BufReader merupakan komponen Rust yang melakukan buffering untuk operasi I/O, sehingga mengurangi jumlah operasi sistem langsung saat membaca data. Ini sangat efisien untuk protokol berbasis teks seperti HTTP.

Proses pengumpulan request HTTP dilakukan melalui serangkaian method chaining:
1) ```.lines()``` – Membaca stream baris per baris, menghasilkan iterator bertipe ```Result<String>```.

2) ```.map(|result| result.unwrap())``` – Mengubah setiap ```Result``` menjadi ```String``` dengan ```unwrap()```. Ini berisiko jika terjadi error, namun cocok untuk tujuan pembelajaran.

3) ```.take_while(|line| !line.is_empty())``` – Mengambil baris hingga menemui baris kosong, sesuai spesifikasi HTTP di mana baris kosong menandai akhir header.

4) ```.collect()``` – Mengumpulkan semua baris ke dalam ```Vec<String>```.

Setelah menjalankan ```cargo run``` dan mengakses ```http://127.0.0.1:7878```, server menampilkan output berikut:
```
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "Connection: keep-alive",
    "sec-ch-ua: \"Chromium\";v=\"134\", \"Not:A-Brand\";v=\"24\", \"Microsoft Edge\";v=\"134\"",
    "sec-ch-ua-mobile: ?0",
    "sec-ch-ua-platform: \"Windows\"",
    "Upgrade-Insecure-Requests: 1",
    "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 Edg/134.0.0.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Dest: document",
    "Accept-Encoding: gzip, deflate, br, zstd",
    "Accept-Language: en-US,en;q=0.9,id;q=0.8",
    "Cookie: csrftoken=I76dC3qrvXIo7QQFLmTye3j9EZ06Pjup",
]
```

Output menunjukkan client mengirim permintaan HTTP GET ke path utama (/) menggunakan HTTP 1.1. Header Host: 127.0.0.1:7878 menegaskan server lokal yang diakses. User-Agent mengidentifikasi browser client sebagai Microsoft Edge 134 di Windows. Header keamanan seperti sec-ch-ua dan sec-ch-ua-platform memberi detail browser dan OS, sementara Cookie mengandung token CSRF untuk keamanan. Header Accept-Encoding dan Accept-Language menunjukkan preferensi kompresi data dan bahasa yang diinginkan client.

Seluruh data ini dibaca oleh BufReader dari koneksi TCP, berhenti saat menemui baris kosong sesuai standar HTTP. Meski sederhana, kode ini menjadi dasar untuk mempelajari cara server web membaca permintaan HTTP di Rust.

## Commit 2

![Alt text](images/commit2.png)

Berikut perubahan code pada milestone 2:
```
 use std::{
     fs,
     ...
 }
 
 fn handle_connection(mut stream: TcpStream) {
     let buf_reader = BufReader::new(&mut stream);
     let http_request: Vec<_> = buf_reader
         .lines() 
         .map(|result| result.unwrap()) 
         .take_while(|line| !line.is_empty())
         .collect();
 
     let status_line = "HTTP/1.1 200 OK"; 
     let contents = fs::read_to_string("hello.html").unwrap(); 
     let length = contents.len();
 
     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
     stream.write_all(response.as_bytes()).unwrap();
 }
 ```

 Pada kode yang diperbarui, fungsi ```handle_connection``` tidak hanya membaca permintaan HTTP dari klien tetapi juga membangun respons untuk dikirim kembali. Server menggunakan ```BufReader``` untuk membaca stream koneksi TCP baris per baris hingga menemui baris kosong, yang menandai akhir header HTTP. Meskipun permintaan dikumpulkan ke variabel ```http_request```, data ini belum diproses lebih lanjut—menunjukkan bahwa server saat ini merespons semua permintaan dengan cara yang sama, terlepas dari isi request-nya.

Setelah membaca permintaan, server menyiapkan respons HTTP dengan status ```200 OK```, lalu membaca isi file ```hello.html``` menggunakan ```fs::read_to_string```. Header ```Content-Length``` dihitung berdasarkan panjang konten HTML untuk memastikan klien dapat memahami batas data yang diterima. Seluruh respons dikemas dalam format string yang sesuai standar HTTP (status line, header, baris kosong, dan body), lalu dikirim ke klien melalui ```stream.write_all()```. Dengan ini, server mampu mengirim halaman HTML statis ke browser klien sebagai tanggapan atas permintaan apa pun.

Setelah menjalankan ```cargo run``` dan mengakses ```http://127.0.0.1:7878```, server menampilkan output berikut:

```
PS C:\Users\Basti\OneDrive\Desktop\Folders\Kuliah\Semester 4\ADPRO\hello> cargo run
warning: unused variable: `http_request`
  --> src\main.rs:16:9
   |
16 |     let http_request: Vec<_> = buf_reader
   |         ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_http_request`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `hello` (bin "hello") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target\debug\hello.exe`
```
