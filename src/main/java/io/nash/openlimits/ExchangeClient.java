package io.nash.openlimits;


import java.lang.reflect.Array;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.function.Consumer;

public class ExchangeClient {
    static {
        System.loadLibrary("openlimits_java");
    }
    private ExchangeClientConfig config;

    @SuppressWarnings("unused")
    private long _config;

    @SuppressWarnings("unused")
    private long _client;

    @SuppressWarnings("unused")
    private long _runtime;

    @SuppressWarnings("unused")
    private long _sub_tx;

    @SuppressWarnings("unused")
    private long _handler_tx;

    @SuppressWarnings("unused")
    private void onPing() {
        this.onPingCallbacks.forEach(Runnable::run);
    }

    @SuppressWarnings("unused")
    private void onDisconnect() {
        this.onDisconnectCallbacks.forEach(Runnable::run);
    }

    @SuppressWarnings("unused")
    private void onError(OpenLimitsException error) {
        this.onErrorCallbacks.forEach(callback -> callback.accept(error));
    }

    @SuppressWarnings("unused")
    private void onOrderbook(OrderbookResponse orderbook) {
        if (!this.onOrderbookCallbacks.containsKey(orderbook.market)) {
            return;
        }
        this.onOrderbookCallbacks.get(orderbook.market).forEach(callback -> callback.accept(orderbook));

    }
    @SuppressWarnings("unused")
    private void onTrades(String market, Trade[] trades) {
        if (!this.onTradesCallbacks.containsKey(market)) {
            return;
        }
        TradesResponse tradesResponse = new TradesResponse(market, trades);
        this.onTradesCallbacks.get(market).forEach(callback -> callback.accept(tradesResponse));
    }

    final private ArrayList<Consumer<OpenLimitsException>> onErrorCallbacks = new ArrayList<>();
    final private ArrayList<Runnable> onDisconnectCallbacks = new ArrayList<>();
    final private ArrayList<Runnable> onPingCallbacks = new ArrayList<>();
    final private HashMap<String, ArrayList<Consumer<OrderbookResponse>>> onOrderbookCallbacks = new HashMap<>();
    final private HashMap<String, ArrayList<Consumer<TradesResponse>>> onTradesCallbacks = new HashMap<>();

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


    native private void subscribe(ExchangeClient client, Subscription subscription);
    native private void disconnect(ExchangeClient client);

    native private void init(ExchangeClient client, ExchangeClientConfig conf);
    public void subscribeTrades(String market, Consumer<TradesResponse> onTrades) {
        if (!this.onTradesCallbacks.containsKey(market)) {
            this.onTradesCallbacks.put(market, new ArrayList<>());
        }
        this.onTradesCallbacks.get(market).add(onTrades);
        this.subscribe(this, Subscription.trade(market));

    }
    public void subscribeOrderbook(String market, Consumer<OrderbookResponse> onOrderbook) {
        if (!this.onOrderbookCallbacks.containsKey(market)) {
            this.onOrderbookCallbacks.put(market, new ArrayList<>());
        }
        this.onOrderbookCallbacks.get(market).add(onOrderbook);
        this.subscribe(this, Subscription.orderbook(market));
    }
    public void subscribeError(Consumer<OpenLimitsException> onError) {
        this.onErrorCallbacks.add(onError);
    }
    public void subscribePing(Runnable onPing) {
        this.onPingCallbacks.add(onPing);
    }
    public void subscribeDisconnect(Runnable onPing) {
        this.onDisconnectCallbacks.add(onPing);
    }
    public void disconnect() {
        this.disconnect(this);
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

    public static void run() {
        String apiKey = System.getenv("NASH_API_SESSION_PROD");
        String secret = System.getenv("NASH_API_SECRET_PROD");
        NashConfig config = new NashConfig(
                new NashCredentials(secret, apiKey),
                0,
                "production",
                1000
        );
        final ExchangeClient client = new ExchangeClient(new ExchangeClientConfig(config));


        client.subscribeTrades("btc_usdc", (TradesResponse trades) -> {
            System.out.println(trades);
        });

        client.subscribeError(err -> {
            System.out.println("Experienced an error, cleaning up");
            client.cancelAllOrders(new CancelAllOrdersRequest("btc_usdc"));
            client.cancelAllOrders(new CancelAllOrdersRequest("noia_usdc"));
            client.disconnect();
        });

        client.subscribeDisconnect(() -> {
            System.out.println("Resetting bot");
        });
    }

    public static void main(String[] args) {
        run();
    }
}
