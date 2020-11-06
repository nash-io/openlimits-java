package io.nash.openlimits;


public class ExchangeClient {
    static {
        System.loadLibrary("openlimits_java");
    }
    private ExchangeClientConfig config;
    private long _config;
    private long _client;
    private long _runtime;
    private long _sub_tx;
    private long _handler_tx;
    private OpenLimitsEventHandler eventHandler = null;

    native private OrderbookResponse orderBook(ExchangeClient client, String market);
    native private Ticker getPriceTicker(ExchangeClient client, String market);
    native private Candle[] getHistoricRates(ExchangeClient client, GetHistoryRatesRequest request);
    native private Trade[] getHistoricTrades(ExchangeClient client, GetHistoryTradeRequest request);
    native private Order limitBuy(ExchangeClient client, LimitRequest request);
    native private Order limitSell(ExchangeClient client, LimitRequest request);
    native private Order marketBuy(ExchangeClient client, MarketRequest request);
    native private Order marketSell(ExchangeClient client, MarketRequest request);
    native private Order getOrder(ExchangeClient client, GetOrderRequest request);
    native private Order[] getOrderHistory(ExchangeClient client, GetOrderHistoryRequest request);
    native private Order[] getAllOpenOrders(ExchangeClient client);
    native private Trade[] getTradeHistory(ExchangeClient client, TradeHistoryRequest request);
    native private Balance[] getAccountBalances(ExchangeClient client, Paginator paginator);

    native private OrderCanceled cancelOrder(ExchangeClient client, CancelOrderRequest req);
    native private OrderCanceled[] cancelAllOrders(ExchangeClient client, CancelAllOrdersRequest req);
    native private void setSubscriptionCallback(ExchangeClient client);

    native private MarketPair[] receivePairs(ExchangeClient client);


    native private void subscribe(ExchangeClient client, Subscription subscription);
    native private void disconnect(ExchangeClient client);

    native private void init(ExchangeClient client, ExchangeClientConfig conf);
    public void subscribe(Subscription subscription) {
        this.subscribe(this, subscription);
    }
    public void disconnect() {
        this.disconnect(this);
    }

    public void setSubscriptionCallback(OpenLimitsEventHandler handler) {
        this.eventHandler = handler;
        this.setSubscriptionCallback(this);
    }

    public Order limitBuy(LimitRequest request) {
        return this.limitBuy(this, request);
    }
    public Order limitSell(LimitRequest request) {
        return this.limitSell(this, request);
    }
    public Order marketBuy(MarketRequest request) {
        return this.marketBuy(this, request);
    }
    public Order marketSell(MarketRequest request) {
        return this.marketSell(this, request);
    }
    public Order getOrder(GetOrderRequest request) {
        return this.getOrder(this, request);
    }
    public Order[] getAllOpenOrders() {
        return this.getAllOpenOrders(this);
    }
    public Order[] getOrderHistory(GetOrderHistoryRequest request) {
        return this.getOrderHistory(this, request);
    }
    public OrderbookResponse orderBook(String market) {
        return this.orderBook(this, market);
    }
    public Ticker getPriceTicker(String market) {
        return this.getPriceTicker(this, market);
    }
    public  Candle[] getHistoricRates(GetHistoryRatesRequest request) {
        return this.getHistoricRates(this, request);
    }
    public  Trade[] getHistoricTrades(GetHistoryTradeRequest request) {
        return this.getHistoricTrades(this, request);
    }
    public Trade[] getTradeHistory(TradeHistoryRequest request) {
        return this.getTradeHistory(this, request);
    }
    public Balance[] getAccountBalances(Paginator paginator) {
        return this.getAccountBalances(this, paginator);
    }
    public OrderCanceled cancelOrder(CancelOrderRequest req) {
        return this.cancelOrder(this, req);
    }
    public OrderCanceled[] cancelAllOrders(CancelAllOrdersRequest req) {
        return this.cancelAllOrders(this, req);
    }
    public MarketPair[] receivePairs() {
        return this.receivePairs(this);
    }
    public ExchangeClient(ExchangeClientConfig conf) {
        this.config = conf;
        this.init(this, conf);
    }

    public static void run(Runnable restart) {
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
        final ExchangeClient client = new ExchangeClient(new ExchangeClientConfig(nashConfig));
        client.setSubscriptionCallback(new OpenLimitsEventHandler() {
            @Override
            public void onOrderbook(OrderbookResponse orderbook) {
                System.out.println(orderbook);
            }
            @Override
            public void onError() {
                System.out.println("Got error. Closing down clients");
                restart.run();
            }
        });
        client.subscribe(Subscription.orderbook("btc_usdc", 5));
    }

    public static void main(String[] args) {
        run(() -> {
            main(args);
        });
    }
}
