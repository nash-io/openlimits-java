package io.nash.openlimits;

import junit.framework.TestCase;

import java.util.Arrays;

public class BinanceExchangeClientTest extends TestCase {
    static ExchangeClient client;

    public void setUp() throws Exception {
        super.setUp();
        String apiKey = System.getenv("BINANCE_API_KEY");
        String secret = System.getenv("BINANCE_API_SECRET");

        client = new ExchangeClient(new ExchangeClientConfig(new BinanceConfig(
                true,
                new BinanceCredentials(
                        apiKey,
                        secret
                )
        )));
    }
    public void testOrderBook() {
        System.out.println(client.orderBook("BNBBTC"));
    }
    public void testPriceTicker() {
        System.out.println(client.getPriceTicker("BNBBTC"));
    }
    public void testGetHistoryRates() {
        System.out.println(Arrays.toString(client.getHistoricRates( new GetHistoryRatesRequest(
                "BNBBTC",
                "OneHour"
        ))));
    }
}