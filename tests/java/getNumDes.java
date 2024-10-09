import java.io.File;

import dutchman.mil.nitfgnr;

public class getNumDes {
    public static void main(String[] args) {
        File file = new File("/opt/nitf-gnr/tests/nitf/Japan_1_Uncompressed.ntf");
        if (!file.exists()) {
            System.out.println("File not found");
            return;
        } 
        int numdes  = nitfgnr.getNumDes(file);
        System.out.println(numdes);
    }
}
