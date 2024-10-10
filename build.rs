use std::process::Command;

fn main() {
    Command::new("javac")
        .args(["-source", "1.8", "-target", "1.8", "java/src/dutchman/mil/nitfgnr.java", "-d", "java/out/"])
        .status()
        .expect("Failed to compile Java code");
    Command::new("jar")
        .args(["cfe", "java/jar/nitfgnr.jar", "dutchman.mil.nitfgnr", "-C", "java/out", "dutchman"])
        .status()
        .expect("Failed to create JAR file");
    Command::new("javadoc")
        .args(["-d", "java/docs", "java/src/dutchman/mil/nitfgnr.java"])
        .status()
        .expect("Failed to generate javadocs");
    Command::new("jar")
        .args(["uf", "java/jar/nitfgnr.jar", "-C", "java/docs", "."])
        .status()
        .expect("Failed to add javadocs to the JAR");
}

// javac nitfgnr.java
// jar cf nitfgnr.jar nitfgnr.class
// javac -cp .:nitfgnr.jar javatest.java
