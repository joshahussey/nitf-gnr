use std::process::Command;

fn main() {
    Command::new("javac")
        .args(["-source", "1.8", "-target", "1.8", "java/src/dutchman/mil/nitfgnr.java", "-d", "java/out/"])
        .status()
        .expect("Failed to compile Java code");
    Command::new("jar")
        .args(["-source", "1.8", "-target", "1.8","cfe", "java/jar/nitfgnr.jar", "dutchman.mil.nitfgnr", "-C", "java/out", "dutchman"])
        .status()
        .expect("Failed to create JAR file");
}

// javac nitfgnr.java
// jar cf nitfgnr.jar nitfgnr.class
// javac -cp .:nitfgnr.jar javatest.java
