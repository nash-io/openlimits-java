package io.nash.openlimits;

public class LimitRequest {
    public final String price;
    public final String size;
    public final String market;
    public LimitRequest(String amount, String size, String market) {
        this.price = amount;
        this.size = size;
        this.market = market;
    }

    @Override
    public String toString() {
        return "LimitRequest{" +
                "amount='" + price + '\'' +
                ", size='" + size + '\'' +
                ", market='" + market + '\'' +
                '}';
    }
}
