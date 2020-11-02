package io.nash.openlimits;


public class ExchangeClient {
    static {
        System.loadLibrary("openlimits_java");
    }
    private long _client;
    private long _runtime;
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

    native private MarketPair[] receivePairs(ExchangeClient client);

    native private int init(ExchangeClient client, ExchangeClientConfig conf);
    private ExchangeClientConfig config;

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
        this.init(this, this.config);
    }

    static public void main(String[] args) {
        String apiKey = System.getenv("BINANCE_API_KEY");
        String secret = System.getenv("BINANCE_API_SECRET");

        ExchangeClient client = new ExchangeClient(new ExchangeClientConfig(new BinanceConfig(
                true,
                new BinanceCredentials(
                        apiKey,
                        secret
                )
        )));
        System.out.println("Hello?");
        try {
            client.getAllOpenOrders();
        } catch(RuntimeException e){
            System.out.println("Hello?");
        }

    }
}
