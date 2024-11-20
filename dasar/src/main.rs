fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    assert_eq!(1 + 1, 2);
}

#[test]
fn variable_rust(){
    let nama = "alexander zulfikar";

    let mut umur = 20;
    umur = 33;

    println!("hallo {:?}", nama);
    println!("umur {:?}", umur);

}

// number
#[test]
fn variable_number(){
    let numberone: usize = 23;
//     number bisa explisit, tidak bisa konversi ke versi lebih rendah, bisa ke lebih tinggi
//     mungkin cocok untuk management memory

    println!("{:?}", numberone);
}

// bool
// bool di rust nggak ada bedanya dengan bool di bahasa pemograman lain
#[test]
fn variable_bool(){
    let boool: bool = false;
    println!("{:?}", boool);
}


// perbandingan juga sama dengan bahasa pemograman lain
// yang cukup beda emg number yang ada tipe tipe nya awkwk
#[test]
fn variable_perbandingan(){
    let umur_jawir = 20;
    let  umur_jono = 30;

    let kebenaran = umur_jawir > umur_jono;

    println!("{:?}", kebenaran);
}


#[test]
// type data character yang cuma ada 1 char jika lebih eror entahlah ini guna atau kagak wkwk
fn veriable_char(){
    let charku = 's';

    println!("{:?}", charku);
}

// nah udah semua tipe ada scalar atau single point selanjutnya kita akan ke data compound awkwk


#[test]
fn variable_tuple(){

    let person: (&str,isize,&str) = ("alexander",22,"sinomwidodo");


    println!("{:?}", person);
//     tupple juga bisa kosong contoh nya kayak function ini yang mana () adalah kosong tidak ada return atau param nya
//     menarik si ada ginian di rust

}


#[test]
fn variable_array(){
    let list_pekerja: [&str;1] = ["qwe"];
    let list_umur_pekerja = [[1,2,3],[1,2,3]];

    println!("{:?}", list_pekerja);
    println!("{:?}", list_umur_pekerja);

}




