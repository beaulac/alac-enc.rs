extern crate cpp_to_rust_generator;
extern crate tempdir;

use cpp_to_rust_generator::common::file_utils::{PathBufWithAdded, create_dir, create_dir_all};
use cpp_to_rust_generator::common::utils::{run_command, add_env_path_item};
use cpp_to_rust_generator::common::cpp_lib_builder::{CppLibBuilder, BuildType};
use cpp_to_rust_generator::common::errors::fancy_unwrap;
use cpp_to_rust_generator::config::{Config, CrateProperties, CacheUsage};
use cpp_to_rust_generator::common::cpp_build_config::CppBuildConfigData;
use cpp_to_rust_generator::common::target;
use std::process::Command;
use std::path::{Path, PathBuf};


fn build_cpp_lib() -> PathBuf {
    let cpp_lib_source_dir = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("alac");
        path.push("cpp");
        path
    };
    assert!(cpp_lib_source_dir.exists());

    let output_dir = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("output");
        path
    };

    let build_dir = output_dir.as_path().join("intermediate");

    let install_dir = output_dir.as_path().join("install");

    if !build_dir.exists() {
        create_dir(&build_dir).unwrap();
    }
    if !install_dir.exists() {
        create_dir(&install_dir).unwrap();
    }
    fancy_unwrap(CppLibBuilder {
        cmake_source_dir: cpp_lib_source_dir,
        build_dir: build_dir,
        build_type: BuildType::Release,
        install_dir: install_dir,
        num_jobs: None,
        cmake_vars: Vec::new(),
    }
        .run());
    output_dir
}

fn main() {
    let output_dir = build_cpp_lib();
    let crate_dir = output_dir.as_path().join("crate");
    let cpp_install_lib_dir = output_dir.as_path().join("install/lib");
    assert!(cpp_install_lib_dir.exists());
    let crate_properties = CrateProperties::new("rust_alac", "0.0.0");

    let mut config = Config::new(&crate_dir,
                                 output_dir.as_path().join("cache"),
                                 crate_properties);
    config.add_include_directive("alac_all.h");
    let include_path = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("alac");
        path.push("cpp");
        path.push("include");
        path
    };
    let crate_template_path = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("alac");
        path.push("crate");
        path
    };
    assert!(include_path.exists());
    config.add_include_path(&include_path);
    config.add_target_include_path(&include_path);

    {
        let mut data = CppBuildConfigData::new();
        data.add_linked_lib("alac");
        config
            .cpp_build_config_mut()
            .add(target::Condition::True, data);
    }
    {
        let mut data = CppBuildConfigData::new();
        data.add_compiler_flag("-fPIC");
        data.add_compiler_flag("-std=gnu++11");
        config
            .cpp_build_config_mut()
            .add(target::Condition::Env(target::Env::Msvc).negate(), data);
    }
    if target::current_env() == target::Env::Msvc {
        config.add_cpp_parser_argument("-std=c++14");
    } else {
        config.add_cpp_parser_argument("-std=gnu++11");
    }
    config.set_crate_template_path(&crate_template_path);
    config.set_cache_usage(CacheUsage::None);

    config.set_write_dependencies_local_paths(false);

    fancy_unwrap(config.exec());
    assert!(crate_dir.exists());

    for cargo_cmd in &["update", "build"] {
        let mut command = Command::new("cargo");
        command.arg(cargo_cmd);
        command.arg("-v");
        if *cargo_cmd != "update" {
            command.arg("-j1");
        }
        //    if *cargo_cmd == "test" {
        //      command.arg("--");
        //      command.arg("--nocapture");
        //    }
        command.current_dir(&crate_dir);
        command.env("CPP_TO_RUST_INCLUDE_PATHS", &include_path);
        command.env("CPP_TO_RUST_LIB_PATHS", &cpp_install_lib_dir);
        command.env("PATH",
                    add_env_path_item("PATH", vec![cpp_install_lib_dir.clone()]).unwrap());
        command.env("LD_LIBRARY_PATH",
                    add_env_path_item("LD_LIBRARY_PATH", vec![cpp_install_lib_dir.clone()]).unwrap());
        run_command(&mut command).unwrap();
    }
}