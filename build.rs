use cc;
fn main() {
    cc::Build::new().file("c/macos.h")
    .compile("libgetinfo");
    println!("cargo:rerun-if-changed=c/macos.h")
}
