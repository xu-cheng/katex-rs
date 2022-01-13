fn main() {
    println!("cargo:rerun-if-changed=js/");
    println!("cargo:rerun-if-changed=vendor/");
}
