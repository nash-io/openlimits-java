package io.nash.openlimits;

import java.util.Arrays;

public class OrderbookResponse {
    public final long lastUpdateId;
    public final AskBid asks[];
    public final AskBid bids[];

    public OrderbookResponse(AskBid[] asks, AskBid[] bids, long lastUpdateId) {
        this.asks = asks;
        this.bids = bids;
        this.lastUpdateId = lastUpdateId;
    }

    @Override
    public String toString() {
        return "OrderbookResponse{" +
                "lastUpdateId=" + lastUpdateId +
                ", asks=" + Arrays.toString(asks) +
                ", bids=" + Arrays.toString(bids) +
                '}';
    }
}
