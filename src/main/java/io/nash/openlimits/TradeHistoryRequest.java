package io.nash.openlimits;

public class TradeHistoryRequest {
    public final String market;
    public final String orderId;
    public final Paginator paginator;

    public TradeHistoryRequest(String market, String orderId, Paginator paginator) {
        this.market = market;
        this.orderId = orderId;
        this.paginator = paginator;
    }

    @Override
    public String toString() {
        return "TradeHistoryRequest{" +
                "market='" + market + '\'' +
                ", orderId='" + orderId + '\'' +
                ", paginator=" + paginator +
                '}';
    }
}
