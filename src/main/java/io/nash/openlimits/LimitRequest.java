package io.nash.openlimits;

public class LimitRequest {
    public final String price;
    public final String size;
    public final String market;
    public final String timeInForce;
    public final String clientOrderId;
    public final long timeInForceDurationMs;
    public final boolean postOnly;


    private LimitRequest(String price, String size, String market, String timeInForce, long timeInForceDurationMs, boolean postOnly, String clientOrderId) {
        this.price = price;
        this.size = size;
        this.market = market;
        this.timeInForce = timeInForce;
        this.timeInForceDurationMs = timeInForceDurationMs;
        this.postOnly = postOnly;
        this.clientOrderId = clientOrderId;
    }


    public static LimitRequest immediateOrCancel(String price, String size, String market) {
        return new LimitRequest(price, size, market, "IOC", 0, false, null);
    }
    public static LimitRequest goodTillCancelled(String price, String size, String market) {
        return new LimitRequest(price, size, market, "GTC", 0, false, null);
    }
    public static LimitRequest fillOrKill(String price, String size, String market) {
        return new LimitRequest(price, size, market, "FOK", 0, false, null);
    }
    public static LimitRequest goodTillTIme(String price, String size, String market, long timeInForceDurationMs) {
        return new LimitRequest(price, size, market, "GTT", timeInForceDurationMs, false, null);
    }

    public static LimitRequest immediateOrCancel(String price, String size, String market, String clientOrderId) {
        return new LimitRequest(price, size, market, "IOC", 0, false, clientOrderId);
    }
    public static LimitRequest goodTillCancelled(String price, String size, String market, String clientOrderId) {
        return new LimitRequest(price, size, market, "GTC", 0, false, clientOrderId);
    }
    public static LimitRequest fillOrKill(String price, String size, String market, String clientOrderId) {
        return new LimitRequest(price, size, market, "FOK", 0, false, clientOrderId);
    }
    public static LimitRequest goodTillTIme(String price, String size, String market, long timeInForceDurationMs, String clientOrderId) {
        return new LimitRequest(price, size, market, "GTT", timeInForceDurationMs, false, clientOrderId);
    }


}
