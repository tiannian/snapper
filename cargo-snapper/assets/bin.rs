fn main() {
    println!("cargo:rerun-if-changed=contracts/");
    println!("cargo:rerun-if-changed=scripts/");
    println!("cargo:rerun-if-changed=build.rs");
}
