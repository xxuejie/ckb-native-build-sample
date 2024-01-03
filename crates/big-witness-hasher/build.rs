fn main() {
    println!("cargo:rerun-if-changed=c.c");

    let clang = match std::env::var_os("CLANG") {
        Some(val) => val,
        None => "clang-16".into(),
    };

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "riscv64" {
        cc::Build::new()
            .file("c.c")
            .static_flag(true)
            .include("../../deps/ckb-c-stdlib")
            .include("../../deps/ckb-c-stdlib/libc")
            .compiler(clang)
            .no_default_flags(true)
            .flag("--target=riscv64")
            .flag("-march=rv64imc_zba_zbb_zbc_zbs")
            .flag("-O3")
            .flag("-nostdinc")
            .flag("-nostdlib")
            .flag("-fvisibility=hidden")
            .flag("-fdata-sections")
            .flag("-ffunction-sections")
            .flag("-Wall")
            .flag("-Werror")
            .flag("-Wno-unused-parameter")
            .define("CKB_DECLARATION_ONLY", None)
            .compile("c-impl");
    }
}
