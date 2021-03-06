use std::env;
use std::path::PathBuf;

fn main() {
    if env::var("DOCS_RS").is_ok() {
        // Skip everything when building docs on docs.rs
        return;
    }

    // Account for cross-compilation
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Set CC environment variable to choose alternative C compiler.
    // Optimization level depends on whether or not --release is passed
    // or implied.
    if target_env == "msvc" && env::var("CC").is_err() && which::which("clang-cl").is_ok() {
        env::set_var("CC", "clang-cl");
    }
    let mut cc = cc::Build::new();
    cc.extra_warnings(true);
    cc.warnings_into_errors(true);
    let mut files = vec![PathBuf::from("src/cpu/sloth256_189.c")];

    if cfg!(feature = "no-asm") {
        println!("Compiling without assembly module");
        cc.define("__SLOTH256_189_NO_ASM__", None);
    } else if target_arch == "x86_64" {
        if target_env == "msvc" {
            files.push(PathBuf::from("src/cpu/win64/mod256-189-x86_64.asm"));
        } else {
            files.push(PathBuf::from("src/cpu/assembly.S"));
        }
    }

    if target_family == "wasm" {
        cc.archiver("llvm-ar");
    }

    cc
        // avoid costly transitions
        .flag_if_supported("-mno-avx")
        .flag_if_supported("-fno-builtin-memcpy")
        .flag_if_supported("-Wno-unused-command-line-argument");

    cc.files(&files).compile("sloth256_189");

    if target_os == "windows" && target_env != "msvc" {
        return;
    }

    if cfg!(feature = "cuda") {
        cc::Build::new()
            .cuda(true)
            .cudart("static")
            .extra_warnings(true)
            .warnings_into_errors(true)
            .file("src/cuda/ptx.cu")
            .compile("sloth256_189_cuda");
    }

    if cfg!(feature = "opencl") {
        env::var("DEP_OPENMP_FLAG")
            .unwrap()
            .split(' ')
            .for_each(|f| {
                cc.flag(f);
            });

        println!("cargo:rustc-link-lib=OpenCL");

        let mut cuda_include: String = "".to_string();
        if let Ok(cuda_path) = env::var("CUDA_PATH") {
            println!("cargo:rustc-link-search={}/lib/x64", cuda_path);

            cuda_include = cuda_path + "/include";
        }

        cc::Build::new()
            .cpp(true)
            .flag_if_supported("-pthread")
            .flag_if_supported("-fopenmp")
            .flag_if_supported("/openmp")
            .flag_if_supported("-std:c++17")
            .flag_if_supported("/EHsc")
            .flag_if_supported("-std=c++17")
            .include(cuda_include)
            .file("src/opencl/opencl.cpp")
            .compile("sloth256_189_opencl");
    }
}
