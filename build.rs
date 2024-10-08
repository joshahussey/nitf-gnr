use std::process::Command;

fn main() {
    Command::new("javac")
        .args(["java/src/nitfgnr.java", "-d", "java/out/"])
        .status()
        .expect("Failed to compile Java code");
    Command::new("jar")
        .args(["cf", "java/out/nitfgnr.jar", "java/out/nitfgnr.class"])
        .status()
        .expect("Failed to create JAR file");
}

// javac nitfgnr.java
// jar cf nitfgnr.jar nitfgnr.class
// javac -cp .:nitfgnr.jar javatest.java
