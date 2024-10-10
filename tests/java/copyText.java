import dutchman.mil.nitfgnr;

public class copyText {
    public static void main(String[] args) {
        String input = new String("/opt/nitf-gnr/tests/nitf/copyText.ntf");
        String output = new String("/opt/nitf-gnr/tests/out/copyText.ntf");
        nitfgnr.copyTextSegmentsFromPaths(input, output);
    }
}
