package io.nash.openlimits;

public class MarketRequest {
    public final String size;
    public final String market;
    public MarketRequest(String size, String market) {
        this.size = size;
        this.market = market;
    }

    @Override
    public String toString() {
        return "MarketRequest{" +
                "size='" + size + '\'' +
                ", market='" + market + '\'' +
                '}';
    }
}
