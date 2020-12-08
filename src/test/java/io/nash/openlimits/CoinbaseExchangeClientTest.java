package io.nash.openlimits;

import junit.framework.TestCase;

import java.util.Arrays;

public class CoinbaseExchangeClientTest  extends TestCase  {
    static ExchangeClient client;

    public void setUp() throws Exception {
        super.setUp();
        String apiKey = System.getenv("COINBASE_API_KEY");
        String secret = System.getenv("COINBASE_API_SECRET");
        String passphrase = System.getenv("COINBASE_API_PASSPHRASE");
        CoinbaseConfig config = new CoinbaseConfig(
                true,
                new CoinbaseCredentials(
                        apiKey,
                        secret,
                        passphrase
                )
        );
        client = new ExchangeClient(new ExchangeClientConfig(config));
    }
    public void testPriceTicker() {
        System.out.println(client.getPriceTicker("ETH-BTC"));
    }
    public void testGetHistoryRates() {
        System.out.println(Arrays.toString(client.getHistoricRates(new GetHistoryRatesRequest("ETH-BTC", "OneHour"))));
    }
    public void testOrderBook() {
        System.out.println(client.orderBook("ETH-BTC"));
    }
    public void testGetOrderHistory() {
        System.out.println(Arrays.toString(client.getOrderHistory(new GetOrderHistoryRequest("ETH-BTC"))));
    }
    public void testGetBalances() {
        System.out.println(Arrays.toString(client.getAccountBalances(null)));
    }
    public void testReceivePairs() {
        System.out.println(Arrays.toString(client.receivePairs()));
    }
    public void testTrading() {
        client.limitBuy(LimitRequest.goodTillCancelled(
                "0.001",
                "1",
                "ETH-BTC"
        ));
        client.limitSell(LimitRequest.goodTillCancelled(
                "0.001",
                "1",
                "ETH-BTC"
        ));

        client.cancelAllOrders(
                new CancelAllOrdersRequest("ETH-BTC")
        );

    }
}
