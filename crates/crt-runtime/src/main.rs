#[link(name = "crt_alloc", kind = "static")]
extern "C" {}

#[link(name = "crt_std", kind = "static")]
extern "C" {}

// cr.o
extern "C" {
    fn _crt_start() -> i32;
}

fn main() {
    //
    let mut v = Vec::new();
    for i in 0..100_000_000_i64 {
        v.push(Box::new(i));
    }
    //

    let ret = unsafe { _crt_start() };
    println!("EXIT");
    std::process::exit(ret);
}
