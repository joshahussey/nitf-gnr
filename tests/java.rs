use nitf_gnr::modify::core::{get_numdes, get_nums, get_numt};
use std::fs;
use std::process::Command;
use std::str;
mod helpers;

#[test]
pub fn build_java_tests() {
    println!("Building Java Tests with javac");
    let java_files: Vec<String> = fs::read_dir("tests/java/")
        .expect("Failed to read java directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "java" {
                Some(path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();
    let output = Command::new("javac")
        .args(["-cp", "java/out/:nitfgnr.jar"])
        .args(&java_files)
        .args(["-d", "tests/java/out/"])
        .output()
        .expect("Failed to compile java test");
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Could not read stderr");
        println!("{}", stderr);
    }
    assert!(output.status.success());
}

#[test]
pub fn get_version() {
    let output = Command::new("java")
        .args([
            "--add-opens",
            "java.base/java.io=ALL-UNNAMED",
            "-D.java.library.path=target/debug",
            "-cp",
            "tests/java/out:java/out/:nitfgnr.jar",
            "tests/java/getVersion.java",
        ])
        .output()
        .expect("Failed to run GetVersion");
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Could not read stderr");
        println!("{}", stderr);
    }
    let stdout = str::from_utf8(&output.stdout).unwrap();
    println!("{}", stdout);
    assert_eq!(stdout.trim(), "NITF02.10");
}

#[test]
pub fn copy_des() {
    let valid_num_des = {
        let file = fs::File::open("tests/out/copyDes.ntf").expect("Failed to open file");
        let file2 = fs::File::open("tests/nitf/copyDes.ntf").expect("Failed to open file");
        let num_des_pre = get_numdes(&file);
        let num_des_add = get_numdes(&file2);
        num_des_pre + num_des_add
    };
    let output = Command::new("java")
        .args([
            "--add-opens",
            "java.base/java.io=ALL-UNNAMED",
            "-D.java.library.path=target/debug",
            "-cp",
            "tests/java/out:java/out/:nitfgnr.jar",
            "tests/java/copyDes.java",
        ])
        .output()
        .expect("Failed to run copyDes");
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Could not read stderr");
        println!("{}", stderr);
    }
    let stdout = str::from_utf8(&output.stdout).unwrap();
    println!("{}", stdout);
    let file = fs::File::open("tests/out/copyDes.ntf").expect("Failed to open file");
    let num_des_post = get_numdes(&file);
    assert_eq!(num_des_post, valid_num_des);
}

#[test]
pub fn copy_text() {
    let valid_numt = {
        let file = fs::File::open("tests/out/copyText.ntf").expect("Failed to open file");
        let file2 = fs::File::open("tests/nitf/copyText.ntf").expect("Failed to open file");
        let numt_pre = get_numt(&file);
        let numt_add = get_numt(&file2);
        numt_pre + numt_add
    };
    let output = Command::new("java")
        .args([
            "--add-opens",
            "java.base/java.io=ALL-UNNAMED",
            "-D.java.library.path=target/debug",
            "-cp",
            "tests/java/out:java/out/:nitfgnr.jar",
            "tests/java/copyText.java",
        ])
        .output()
        .expect("Failed to run copyDes");
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Could not read stderr");
        println!("{}", stderr);
    }
    let stdout = str::from_utf8(&output.stdout).unwrap();
    println!("{}", stdout);
    let file = fs::File::open("tests/out/copyText.ntf").expect("Failed to open file");
    let numt_post = get_numt(&file);
    assert_eq!(numt_post, valid_numt);
}

#[test]
pub fn copy_graphic() {
    let valid_nums = {
        let file = fs::File::open("tests/out/copyGraphic.ntf").expect("Failed to open file");
        let file2 = fs::File::open("tests/nitf/copyGraphic.ntf").expect("Failed to open file");
        let num_g_pre = get_nums(&file);
        let num_g_add = get_nums(&file2);
        num_g_pre + num_g_add
    };
    let output = Command::new("java")
        .args([
            "--add-opens",
            "java.base/java.io=ALL-UNNAMED",
            "-D.java.library.path=target/debug",
            "-cp",
            "tests/java/out:java/out/:nitfgnr.jar",
            "tests/java/copyGraphic.java",
        ])
        .output()
        .expect("Failed to run copyGraphic");
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Could not read stderr");
        println!("{}", stderr);
    }
    let stdout = str::from_utf8(&output.stdout).unwrap();
    println!("{}", stdout);
    let file = fs::File::open("tests/out/copyGraphic.ntf").expect("Failed to open file");
    let num_g_post = get_nums(&file);
    assert_eq!(num_g_post, valid_nums);
}

#[test]
pub fn extract_all_jp2() {
    let output = Command::new("java")
        .args([
            "--add-opens",
            "java.base/java.io=ALL-UNNAMED",
            "-D.java.library.path=target/debug",
            "-cp",
            "tests/java/out:java/out/:nitfgnr.jar",
            "tests/java/extractAllJp2.java",
        ])
        .output()
        .expect("Failed to run extractAllJp2");
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Could not read stderr");
        println!("{}", stderr);
        panic!("Failed to run extractAllJp2");
    }
    let stdout = str::from_utf8(&output.stdout).unwrap();
    println!("{}", stdout);
}

#[test]
pub fn extract_jp2_index() {
    let output = Command::new("java")
        .args([
            "--add-opens",
            "java.base/java.io=ALL-UNNAMED",
            "-D.java.library.path=target/debug",
            "-cp",
            "tests/java/out:java/out/:nitfgnr.jar",
            "tests/java/extractJp2Index.java",
        ])
        .output()
        .expect("Failed to run extractJp2Index");
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Could not read stderr");
        println!("{}", stderr);
    }
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!(stdout.trim(), helpers::calculate_file_crc32("tests/out/extractAllJp20.jp2").unwrap().to_string());
}

#[test]
pub fn get_num_des() {
    let output = Command::new("java")
        .args([
            "--add-opens",
            "java.base/java.io=ALL-UNNAMED",
            "-D.java.library.path=target/debug",
            "-cp",
            "tests/java/out:java/out/:nitfgnr.jar",
            "tests/java/getNumDes.java",
        ])
        .output()
        .expect("Failed to run getNumDes");
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Could not read stderr");
        println!("{}", stderr);
    }
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!(stdout.trim(), "3");
}
