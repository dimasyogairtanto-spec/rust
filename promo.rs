fn main() {
    let hpp: f64 = 27000.0;
    let harga_jual: f64 = 60000.0;
    let mut diskon: f64 = 0.10;
    let mut total_keuntungan: f64 = 0.0;

    println!("Simulasi Promo 7 Hari\n");

    for hari in 1..=7 {
        // harga diskom
        let harga_setelah_diskon = harga_jual * (1.0 - diskon);

        // pajak
        let pajak = if hari % 2 == 0 {
            0.20   // genap
        } else {
            0.125  // ganjil
        };

        // laba
        let pendapatan_bersih = harga_setelah_diskon * (1.0 - pajak);

        // Hitung keuntungan
        let keuntungan = pendapatan_bersih - hpp;

        // total
        total_keuntungan += keuntungan;

        // Tampilkan hasil per hari
        println!("Hari ke-{}", hari);
        println!("  Diskon              : {:.6}%", diskon * 100.0);
        println!("  Harga setelah diskon: Rp {:.2}", harga_setelah_diskon);
        println!("  Pajak marketplace   : {:.1}%", pajak * 100.0);
        println!("  Pendapatan bersih   : Rp {:.2}", pendapatan_bersih);
        println!("  Keuntungan          : Rp {:.2}", keuntungan);
        println!("----------------------------------------");

        // Diskon turun setengah untuk hari berikutnya
        diskon /= 2.0;
    }

    println!("\nTotal Keuntungan 7 Hari: Rp {:.2}", total_keuntungan);
}
