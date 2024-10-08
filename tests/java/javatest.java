import java.io.File;

public class javatest {
    public static void main(String[] args) {
        File file = new File("/opt/nitf-gnr/testdata/input.nitf");
        File outFile = new File("/opt/nitf-gnr/testdata/output.nitf");
        nitfgnr.getVersion(file);
        nitfgnr.copyDesSegmants(file, outFile);
        int numdes = nitfgnr.getNumDes(file);
        int numdesOut = nitfgnr.getNumDes(outFile);
        System.out.println("Number of Descriptors: " + numdes);
        System.out.println("Number of Descriptors: " + numdesOut);
        File test = new File("/opt/nitf-gnr/java/workplease.ntf");
        int numdesTest = nitfgnr.getNumDes(test);
        System.out.println("Number of Descriptors: " + numdesTest);
    }
}
