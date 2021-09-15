use cc;
fn main() {
    cc::Build::new().file("c/macos.h").warnings(false)
    .compile("libgetinfo");
    println!("cargo:rerun-if-changed=c/macos.h")
}
