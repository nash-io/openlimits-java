package io.nash.openlimits;

import junit.framework.TestCase;

import java.util.Arrays;

public class NashExchangeClientTest extends TestCase {
    static ExchangeClient client;

    public void setUp() throws Exception {
        super.setUp();
        String apiKey = System.getenv("NASH_API_KEY");
        String secret = System.getenv("NASH_API_SECRET");

        NashConfig nashConfig = new NashConfig(
                new NashCredentials(
                        secret,
                        apiKey
                ),
                0,
                "sandbox",
                10000
        );

        client = new ExchangeClient(new ExchangeClientConfig(nashConfig));
    }

    public void testOrderBook() {
        System.out.println(client.orderBook("btc_usdc"));
    }

    /*
    public void testPriceTicker() {
        try {
            System.out.println(client.getPriceTicker( "btc_usdc"));
        } catch(RuntimeException e){
            System.out.println("testPriceTicker failed");
        }
    }
    */

    public void testGetHistoryRates() {
        System.out.println(Arrays.toString(client.getHistoricRates( new GetHistoryRatesRequest(
                "btc_usdc",
                "OneHour"
        ))));
    }

    public void testGetHistoryTrades() {
        System.out.println(Arrays.toString(client.getHistoricTrades( new GetHistoryTradeRequest(
                "btc_usdc"
        ))));
    }
}