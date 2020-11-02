package io.nash.openlimits;

public class CancelAllOrdersRequest {
    public final String market;
    public CancelAllOrdersRequest(String market) {
        this.market = market;
    }

    @Override
    public String toString() {
        return "CancelAllOrdersRequest{" +
                "market='" + market + '\'' +
                '}';
    }
}
