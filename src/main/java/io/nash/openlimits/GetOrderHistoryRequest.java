package io.nash.openlimits;

public class GetOrderHistoryRequest {
    public final String market;
    public final Paginator paginator;

    public GetOrderHistoryRequest(String market, Paginator paginator) {
        this.market = market;
        this.paginator = paginator;
    }

    public GetOrderHistoryRequest(String market) {
        this.market = market;
        this.paginator = null;
    }

    public GetOrderHistoryRequest() {
        this.market = null;
        this.paginator = null;
    }
}
