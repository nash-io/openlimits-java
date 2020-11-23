package io.nash.openlimits;

import java.util.Arrays;

public class OrderbookResponse {
    public final String market;
    public final long lastUpdateId;
    public final AskBid asks[];
    public final AskBid bids[];
    public OrderbookResponse(String market, AskBid[] asks, AskBid[] bids, long lastUpdateId) {
        this.market = market;
        this.lastUpdateId = lastUpdateId;
        this.asks = asks;
        this.bids = bids;
    }

    @Override
    public String toString() {
        return "OrderbookResponse{" +
                "market='" + market + '\'' +
                ", lastUpdateId=" + lastUpdateId +
                ", asks=" + Arrays.toString(asks) +
                ", bids=" + Arrays.toString(bids) +
                '}';
    }
}
