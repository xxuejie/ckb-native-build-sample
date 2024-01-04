fn main() {
    println!("cargo:rerun-if-changed=../../deps/lib-dummy-atomics/atomics.c");

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "riscv64" {
        let mut build = cc::Build::new();
        assert!(
            build.get_compiler().is_like_clang(),
            "Clang must be used as the compiler!"
        );
        build
            .file("../../deps/lib-dummy-atomics/atomics.c")
            .static_flag(true)
            .include("../../deps/ckb-c-stdlib")
            .include("../../deps/ckb-c-stdlib/libc")
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
            .compile("dummy-atomics");
    }
}
