package io.nash.openlimits;

public class Subscription {
    public final String tag;
    public final String market;
    public final long depth;

    private Subscription(String tag, String arg1, long args2) {
        this.tag = tag;
        this.market = arg1;
        this.depth = args2;
    }

    public static Subscription orderbook(String market, long depth) {
        return new Subscription("OrderBook", market, depth);
    }
    public static Subscription trade(String market, long depth) {
        return new Subscription("Trade", market, depth);
    }
}
