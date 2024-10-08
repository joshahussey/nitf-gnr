import java.util.zip.CRC32;
import java.util.zip.Checksum;

public class extractJp2Index {
    public static void main(String[] args) {
        String input = new String("/opt/nitf-gnr/tests/nitf/Japan_1_Uncompressed.ntf");
        byte[] bytes = nitfgnr.extractJp2Index(input, 0);
        long checksum = check(bytes);
        System.out.println(checksum);


    }
    public static long check(byte[] bytes) {
        Checksum checksum = new CRC32();
        checksum.update(bytes, 0, bytes.length);
        return checksum.getValue();
    }
}

