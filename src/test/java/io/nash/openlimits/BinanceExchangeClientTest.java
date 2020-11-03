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
    public void testLimitBuy() {
        System.out.println(client.limitBuy(LimitRequest.goodTillCancelled(
                "0.001",
                "1",
                "BNBBTC"
        )));
    }
    public void testLimitBuyIOC() {
        System.out.println(client.limitBuy(LimitRequest.immediateOrCancel(
                "0.001",
                "1",
                "BNBBTC"
        )));
    }
    public void testLimitBuyFOK() {
        System.out.println(client.limitBuy(LimitRequest.fillOrKill(
                "0.001",
                "1",
                "BNBBTC"
        )));
    }
    public void testMarketBuy() {
        System.out.println(client.marketBuy(new MarketRequest(
                "1",
                "BNBBTC"
        )));
    }
    public void testMarketSell() {
        System.out.println(client.marketSell(new MarketRequest(
                "1",
                "BNBBTC"
        )));
    }

    public void testCancelOrder() {
        Order order = client.limitSell(LimitRequest.goodTillCancelled(
                "0.001",
                "1",
                "BNBBTC"
        ));
        System.out.println("Cancelling: " + order);
        System.out.println(
                client.cancelOrder(new CancelOrderRequest(order.id, "BNBBTC"))
        );
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
    public void testGetOrderHistory() {
        System.out.println(Arrays.toString(client.getOrderHistory( new GetOrderHistoryRequest(
                "BNBBTC"
        ))));
    }
    public void testGetOrder() {
        Order order = client.limitSell(LimitRequest.goodTillCancelled(
                "0.001",
                "1",
                "BNBBTC"
        ));

        System.out.println(client.getOrder(new GetOrderRequest(order.id, "BNBBTC")));
    }
    public void testGetTradeHistory() {
        System.out.println(Arrays.toString(client.getTradeHistory( new TradeHistoryRequest(
                "BNBBTC",
                null,
                null
        ))));
    }
    public void testGetBalances() {
        System.out.println(Arrays.toString(client.getAccountBalances(null)));
    }


    public void testCancelAllOrders() {
        client.limitSell(LimitRequest.goodTillCancelled(
                "0.001",
                "1",
                "BNBBTC"
        ));
        client.limitSell(LimitRequest.goodTillCancelled(
                "0.001",
                "1",
                "BNBBTC"
        ));
        System.out.println(
                Arrays.toString(client.cancelAllOrders(new CancelAllOrdersRequest("BNBBTC")))
        );
    }
    public void testReceivePairs() {
        System.out.println(Arrays.toString(client.receivePairs()));
    }

}