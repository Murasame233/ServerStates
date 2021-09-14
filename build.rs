use cc;
fn main() {
    cc::Build::new().file("c/macos.c")
    .compile("libgetinfo");
    println!("cargo:rerun-if-changed=c/macos.c")
}
