package io.nash.openlimits;

public class Subscription {
    public final String tag;
    public final String market;

    private Subscription(String tag, String arg1) {
        this.tag = tag;
        this.market = arg1;
    }

    public static Subscription orderbook(String market) {
        return new Subscription("OrderBook", market);
    }
    public static Subscription trade(String market) {
        return new Subscription("Trade", market);
    }
}
