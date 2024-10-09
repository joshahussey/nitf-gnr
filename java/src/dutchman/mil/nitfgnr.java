package dutchman.mil;

import java.io.FileDescriptor;
import java.io.File;
import java.io.FileInputStream;
import java.lang.reflect.Field;

public class nitfgnr {
    static {
        System.loadLibrary("nitf_gnr");
    }
    
    //Wrapper functions
    public static String getVersion(File file) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.getVersion(rawFd);
    }

    public static int getHeaderLength(File file) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.getHeaderLength(rawFd);
    }

    public static int getNumImages(File file) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.getNumImages(rawFd);
    }

    public static int getNumDes(File file) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.getNumDes(rawFd);
    }

    public static byte[] extractDes(File file, int index) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.extractDes(rawFd, index);
    }

    public static byte[] extractDesHeader(File file, int index) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.extractDesHeader(rawFd, index);
    }

    public static void copyDesSegments(File input, File output) {
        long rawInputFd = rawFdFromFile(input);
        long rawOutputFd = rawFdFromFile(output);
        nitfgnr gnr = new nitfgnr();
        gnr.copyDesSegments(rawInputFd, rawOutputFd);
    }
    
    //Native functions no wrapper
    public static native byte[] addDesBytes(byte[] nitfBytes, byte[]desHeaderBytes, byte[] desDataBytes);
    public static native void copyDesSegmentsFromPaths(String input, String output);
    public static native void extractAllJp2(String input_path, String output_path);
    public static native byte[] extractJp2Index(String input_path, int index);


    //Native functions
    private native String getVersion(long fd);
    private native void copyDesSegments(long inputFd, long outputFd);
    private native int getHeaderLength(long fd);
    private native int getNumImages(long fd);
    private native int getNumDes(long fd);
    private native byte[] extractDes(long fd, int index);
    private native byte[] extractDesHeader(long fd, int index);


    //Helper functions
    private static long rawFdFromFile(File file) {
        long rawFd = 0;
        try {
            FileInputStream fis = new FileInputStream(file);
            Field fdField = FileInputStream.class.getDeclaredField("fd");
            fdField.setAccessible(true);
            FileDescriptor fd = (FileDescriptor) fdField.get(fis);
            Field field = FileDescriptor.class.getDeclaredField("fd");
            field.setAccessible(true);
            rawFd = field.getLong(fd);
        } catch (Exception e) {
            e.printStackTrace();
        }
        return rawFd;
    }
}
