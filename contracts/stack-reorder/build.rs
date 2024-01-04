fn main() {
    println!("cargo:rerun-if-changed=bootloader.S");
    println!("cargo:rerun-if-changed=ld_interface.ld");

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "riscv64" {
        cc::Build::new()
            .file("bootloader.S")
            .static_flag(true)
            .no_default_flags(true)
            .flag("--target=riscv64")
            .flag("-march=rv64imc_zba_zbb_zbc_zbs")
            .flag("-O3")
            .compile("bootloader");
    }
}
