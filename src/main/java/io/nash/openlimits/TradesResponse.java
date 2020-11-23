package io.nash.openlimits;

import java.util.ArrayList;

public class TradesResponse {
    public final String market;
    public final ArrayList<Trade> trades;

    public TradesResponse(String market, ArrayList<Trade> trades) {
        this.market = market;
        this.trades = trades;
    }

    @Override
    public String toString() {
        return "TradesResponse{" +
                "market='" + market + '\'' +
                ", trades=" + trades +
                '}';
    }
}
