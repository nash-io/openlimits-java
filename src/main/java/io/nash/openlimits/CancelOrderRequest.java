package io.nash.openlimits;

public class CancelOrderRequest {
    public final String id;
    public final String market;

    public CancelOrderRequest(String id, String market) {
        this.id = id;
        this.market = market;
    }

    public CancelOrderRequest(String id) {
        this.id = id;
        this.market = null;
    }

    @Override
    public String toString() {
        return "CancelOrderRequest{" +
                "id='" + id + '\'' +
                ", market='" + market + '\'' +
                '}';
    }
}
