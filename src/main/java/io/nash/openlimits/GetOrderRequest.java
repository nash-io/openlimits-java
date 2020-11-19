package io.nash.openlimits;

public class GetOrderRequest {
    public final String id;
    public final String market;

    public GetOrderRequest(String id, String market) {
        this.id = id;
        this.market = market;
    }

    public GetOrderRequest(String id) {
        this.id = id;
        this.market = null;
    }
}
