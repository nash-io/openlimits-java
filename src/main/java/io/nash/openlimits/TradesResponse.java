package io.nash.openlimits;

import java.util.Arrays;

public class TradesResponse {
    public final String market;
    public final Trade[] trades;

    public TradesResponse(String market, Trade[] trades) {
        this.market = market;
        this.trades = trades;
    }

    @Override
    public String toString() {
        return "TradesResponse{" +
                "market='" + market + '\'' +
                ", trades=" + Arrays.toString(trades) +
                '}';
    }
}
