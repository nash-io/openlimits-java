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

    public Order limitBuy(LimitRequest request) {
        this.limitBuy(this, request);
    }
    public Order limitSell(LimitRequest request) {
        this.limitSell(this, request);
    }
    public Order marketBuy(MarketRequest request) {
        this.marketBuy(this, request);
    }
    public Order marketSell(MarketRequest request) {
        this.marketSell(this, request);
    }
    native private int init(ExchangeClient client, ExchangeClientConfig conf);
    private ExchangeClientConfig config;

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
    public ExchangeClient(ExchangeClientConfig conf) {
        this.config = conf;
        this.init(this, this.config);
    }
}
