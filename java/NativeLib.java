public class NativeLib {
    static {
        System.loadLibrary("native_lib");  // Load the native library
    }

    // Declare the native method
    public native void extractJp2(String inputPath, String outputPath);

    public static void main(String[] args) {
        NativeLib lib = new NativeLib();
        lib.extractJp2("/opt/nitf-gnr/testdata/input.nitf", "output");
        System.out.println("JP2 extraction completed!");
    }
}

