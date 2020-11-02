package io.nash.openlimits;

public class LimitRequest {
    public final String price;
    public final String size;
    public final String market;
    public LimitRequest(String price, String size, String market) {
        this.price = price;
        this.size = size;
        this.market = market;
    }

    @Override
    public String toString() {
        return "LimitRequest{" +
                "price='" + price + '\'' +
                ", size='" + size + '\'' +
                ", market='" + market + '\'' +
                '}';
    }
}
