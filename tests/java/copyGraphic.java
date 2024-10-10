import dutchman.mil.nitfgnr;

public class copyGraphic {
    public static void main(String[] args) {
        String input = new String("/opt/nitf-gnr/tests/nitf/JBP_FMT_POS_06 (SYM).NTF");
        String output = new String("/opt/nitf-gnr/tests/out/copyGraphic.ntf");
        nitfgnr.copyGraphicSegmentsFromPaths(input, output);
    }
}
