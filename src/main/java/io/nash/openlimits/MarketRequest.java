package io.nash.openlimits;

public class MarketRequest {
    public final String size;
    public final String market;
    public final String clientOrderId;
    public MarketRequest(String size, String market) {
        this.size = size;
        this.market = market;
        this.clientOrderId = null;
    }

    public MarketRequest(String size, String market, String clientOrderId) {
        this.size = size;
        this.market = market;
        this.clientOrderId = clientOrderId;
    }

    @Override
    public String toString() {
        return "MarketRequest{" +
                "size='" + size + '\'' +
                ", market='" + market + '\'' +
                '}';
    }
}
