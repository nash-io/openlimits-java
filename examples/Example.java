import io.nash.openlimits.*;

public class Example {

    public static void main(String[] args) {
        NashConfig config = new NashConfig(
                null,
                0,
                "sandbox",
                1000
        );
        while (true) {
            ExchangeClient client = new ExchangeClient(new ExchangeClientConfig(config));
            client.close();
        }
    }
}