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

    public void testPairs() {
        System.out.println(Arrays.toString(client.receivePairs()));
    }

    public void testGetOrderHistory() {
        System.out.println(Arrays.toString(client.getOrderHistory( new GetOrderHistoryRequest(
                "btc_usdc"
        ))));
    }
    public void testGetTradeHistory() {
        System.out.println(Arrays.toString(client.getTradeHistory( new TradeHistoryRequest(
                "btc_usdc",
                null,
                null
        ))));
    }
    public void testGetBalances() {
        System.out.println(Arrays.toString(client.getAccountBalances(null)));
    }



    public void testOrders() {
        System.out.println(Arrays.toString(client.getOrderHistory(new GetOrderHistoryRequest(
                "btc_usdc"
        ))));
    }

    public void testTrading() {
        System.out.println(client.limitSell(LimitRequest.goodTillCancelled(
                "6500.0",
                "0.10000",
                "btc_usdc"
        )));
        System.out.println(client.limitBuy(LimitRequest.goodTillCancelled(
                "0.0215423",
                "0.10000",
                "eth_btc"
        )));
    }

    public void testCancelOrder() {
        Order order = client.limitSell(LimitRequest.goodTillCancelled(
                "1.0",
                "0.10000",
                "eth_btc"
        ));

        CancelOrderRequest cancelOrderRequest = new CancelOrderRequest(order.id, "eth_btc");
        System.out.println(cancelOrderRequest);
        client.cancelOrder(cancelOrderRequest);
    }

    public void testCancelAllOrders() {
        client.limitSell(LimitRequest.goodTillCancelled(
                "1.0",
                "0.10000",
                "eth_btc"
        ));
        System.out.println(Arrays.toString(client.cancelAllOrders(new CancelAllOrdersRequest(
                "btc_usdc"
        ))));
    }
}