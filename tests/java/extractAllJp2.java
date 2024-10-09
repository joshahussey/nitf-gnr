import dutchman.mil.nitfgnr;

public class extractAllJp2 {
    public static void main(String[] args) {
        String input = new String("/opt/nitf-gnr/tests/nitf/Japan_1_Uncompressed.ntf");
        String output = new String("/opt/nitf-gnr/tests/out/extractAllJp2");;
        nitfgnr.extractAllJp2(input, output);
    }
}
