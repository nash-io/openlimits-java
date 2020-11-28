package io.nash.openlimits;

import java.util.concurrent.Callable;

/**
 * We are wrapping Nash calls to recreate the client after a "Could not register request with broker" error
 * This is neccessary because the library is actually using websockets to make the requests and requires recreating the client if connection is broken
 * When Nash adds an auto-reconnect on the Rust web sockets we should be able to remove this wrapper
 */
public class NashClient {
    NashCredentials credentials;
    ExchangeClient client;

    private void buildClient() {
        client = new ExchangeClient(
                    new ExchangeClientConfig(
                            new NashConfig(
                                    credentials,
                                    0,
                                    "production",
                                    10000,
                                    "2PTzyS"
                            )
                    )
        );
    }

    public NashClient() {
        buildClient();
    }

    public NashClient(String apiKey, String secretKey) {
        credentials = new NashCredentials(secretKey, apiKey);
        buildClient();
    }

    public Order limitBuy(LimitRequest request) {
        return wrapCall((Callable<Order>) () -> client.limitBuy(request));
    }

    public Order limitSell(LimitRequest request) {
        return wrapCall((Callable<Order>) () -> client.limitSell(request));
    }

    public Order marketBuy(MarketRequest request) {
        return wrapCall((Callable<Order>) () -> client.marketBuy(request));
    }

    public Order marketSell(MarketRequest request) {
        return wrapCall((Callable<Order>) () -> client.marketSell(request));
    }

    public Balance[] getAccountBalances(Paginator paginator) {
        return wrapCall((Callable<Balance[]>) () -> client.getAccountBalances(paginator));
    }
    public OrderCanceled cancelOrder(CancelOrderRequest req) {
        return wrapCall((Callable<OrderCanceled>) () -> client.cancelOrder(req));
    }

    public Order getOrder(GetOrderRequest req) {
        return wrapCall((Callable<Order>) () -> client.getOrder(req));
    }

    public MarketPair[] receivePairs() {
        return wrapCall((Callable<MarketPair[]>) () -> client.receivePairs());
    }

    public  Candle[] getHistoricRates(GetHistoryRatesRequest req) {
        return wrapCall((Callable<Candle[]>) () -> client.getHistoricRates(req));
    }

    private <T> T wrapCall(Callable<T> callable) {
        while(true) {
            try {
                return callable.call();
            }
            catch(NashProtocolError error) {
                if (error.getMessage().contains("Could not register request with broker")) {
                    buildClient();
                    try {
                        Thread.sleep(1000);
                    } catch (InterruptedException e) {
                    }
                }
                else {
                    throw error;
                }
            }
            catch(Exception error) {

            }
        }
    }
}
