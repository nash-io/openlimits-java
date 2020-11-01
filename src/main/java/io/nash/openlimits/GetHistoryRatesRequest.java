package io.nash.openlimits;


public class GetHistoryRatesRequest {
    public final String market;
    public final String interval;

    public final Paginator paginator;

    public GetHistoryRatesRequest(String market,  String interval, Paginator paginator) {
        this.market = market;
        this.interval = interval;
        this.paginator = paginator;
    }

    public GetHistoryRatesRequest(String market,  String interval) {
        this.market = market;
        this.interval = interval;
        this.paginator = null;
    }

    @Override
    public String toString() {
        return "GetHistoryRatesRequest{" +
                "market='" + market + '\'' +
                ", interval='" + interval + '\'' +
                ", paginator=" + paginator +
                '}';
    }
}
