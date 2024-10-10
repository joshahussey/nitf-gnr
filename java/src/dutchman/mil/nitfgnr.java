package dutchman.mil;

import java.io.FileDescriptor;
import java.io.File;
import java.io.FileInputStream;
import java.lang.reflect.Field;

/**
 * Provides NITF file handling functionality with methods for reading, copying, 
 * and extracting specific segments from NITF files.
 */
public class nitfgnr {
    static {
        System.loadLibrary("nitf_gnr");
    }
    
    //Wrapper functions
    
    /**
     * Retrieves the version of the NITF file.
     *
     * @param file the NITF file
     * @return the version of the file as a String
     */
    public static String getVersion(File file) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.getVersion(rawFd);
    }

    /**
     * Retrieves the length of the header section in the NITF file.
     *
     * @param file the NITF file
     * @return the length of the header in bytes
     */
    public static int getHeaderLength(File file) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.getHeaderLength(rawFd);
    }

    /**
     * Gets the number of image segments in the NITF file.
     *
     * @param file the NITF file
     * @return the number of image segments
     */
    public static int getNumImages(File file) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.getNumImages(rawFd);
    }

    /**
     * Retrieves the number of Data Extension Segments (DES) in the NITF file.
     *
     * @param file the NITF file
     * @return the number of DES segments
     */
    public static int getNumDes(File file) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.getNumDes(rawFd);
    }

    /**
     * Extracts a specific Data Extension Segment (DES) from the NITF file.
     *
     * @param file the NITF file
     * @param index the index of the DES to extract
     * @return the extracted DES as a byte array
     */
    public static byte[] extractDes(File file, int index) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.extractDes(rawFd, index);
    }

    /**
     * Extracts the header of a specific Data Extension Segment (DES) from the NITF file.
     *
     * @param file the NITF file
     * @param index the index of the DES to extract
     * @return the extracted DES header as a byte array
     */
    public static byte[] extractDesHeader(File file, int index) {
        long rawFd = rawFdFromFile(file);
        nitfgnr gnr = new nitfgnr();
        return gnr.extractDesHeader(rawFd, index);
    }

    /**
     * Copies all Data Extension Segments (DES) from one NITF file to another.
     *
     * @param input the input NITF file
     * @param output the output NITF file
     */
    public static void copyDesSegments(File input, File output) {
        long rawInputFd = rawFdFromFile(input);
        long rawOutputFd = rawFdFromFile(output);
        nitfgnr gnr = new nitfgnr();
        gnr.copyDesSegments(rawInputFd, rawOutputFd);
    }
    
    //Native functions no wrapper
    //public static native byte[] addDesBytes(byte[] nitfBytes, byte[]desHeaderBytes, byte[] desDataBytes);
    
    /**
     * Copies all Data Extension Segments (DES) from one NITF file to another.
     *
     * @param input path to the input NITF file
     * @param output path to the output NITF file
     */
    public static native void copyDesSegmentsFromPaths(String input, String output);

    /**
     * Copies all Graphic Segments from one NITF file to another.
     *
     * @param input path to the input NITF file
     * @param output path to the output NITF file
     */
    public static native void copyGraphicSegmentsFromPaths(String input, String output);

    /**
     * Copies all Text Segments from one NITF file to another.
     *
     * @param input path to the input NITF file
     * @param output path to the output NITF file
     */
    public static native void copyTextSegmentsFromPaths(String input, String output);

    /**
     * Copies all Graphic, Text, and Data Extension Segments (DES) from one NITF file to another.
     *
     * @param input path to input NITF file
     * @param output path to the output NITF file
     */
    public static native void copyGTDSegmentsFromPaths(String input, String output);

    /**
     * Extracts all JPEG2000 (JP2) images from a NITF file.
     *
     * @param input_path path to the input NITF file
     * @param output_path path to the output directory
     */
    public static native void extractAllJp2(String input_path, String output_path);

    /**
     * Extracts a specific JPEG2000 (JP2) image from a NITF file.
     *
     * @param input_path path to the input NITF file
     * @param index the index of the JP2 image to extract
     * @return the extracted JP2 image as a byte array
     */
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
