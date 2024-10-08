import java.io.File;

public class getVersion {
    public static void main(String[] args) {
        File file = new File("/opt/nitf-gnr/tests/nitf/Japan_1_Uncompressed.ntf");
        if (!file.exists()) {
            System.out.println("File not found");
            return;
        } 
        String ver = nitfgnr.getVersion(file);
        System.out.println(ver);
    }
}
