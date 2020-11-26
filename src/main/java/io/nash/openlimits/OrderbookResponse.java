package io.nash.openlimits;

import java.util.Arrays;

public class OrderbookResponse {
    public final String market;
    public final long updateId;
    public final long lastUpdateId;
    public final AskBid asks[];
    public final AskBid bids[];
    public OrderbookResponse(String market, AskBid[] asks, AskBid[] bids, long lastUpdateId, long updateId) {
        this.market = market;
        this.lastUpdateId = lastUpdateId;
        this.updateId = updateId;
        this.asks = asks;
        this.bids = bids;
    }

    @Override
    public String toString() {
        return "OrderbookResponse{" +
                "market='" + market + '\'' +
                ", updateId=" + updateId +
                ", lastUpdateId=" + lastUpdateId +
                ", asks=" + Arrays.toString(asks) +
                ", bids=" + Arrays.toString(bids) +
                '}';
    }
}
