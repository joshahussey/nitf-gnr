public class copyDes {
    public static void main(String[] args) {
        String input = new String("/opt/nitf-gnr/tests/nitf/Japan_1_Uncompressed.ntf");
        String output = new String("/opt/nitf-gnr/tests/out/copyDes.ntf");
        nitfgnr.copyDesSegmantsFromPaths(input, output);
    }
}
