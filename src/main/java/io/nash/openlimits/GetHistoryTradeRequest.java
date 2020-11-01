package io.nash.openlimits;

public class GetHistoryTradeRequest {
    public final String market;
    public final Paginator paginator;

    public GetHistoryTradeRequest(String market, Paginator paginator) {
        this.market = market;
        this.paginator = paginator;
    }
    public GetHistoryTradeRequest(String market) {
        this.market = market;
        this.paginator = null;
    }
}
