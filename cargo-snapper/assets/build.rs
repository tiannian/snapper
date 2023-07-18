fn main() {
    snapper::build().expect("Build snapper project failed");

    println!("cargo:rerun-if-changed=contracts/");
    println!("cargo:rerun-if-changed=scripts/");
    println!("cargo:rerun-if-changed=build.rs");
}
