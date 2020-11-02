package io.nash.openlimits;

abstract public class OpenLimitsEventHandler {
    public void onPing() {

    };
    public void onOrderbook(OrderbookResponse orderbook) {}
    public void onTrades(Trade[] trades) {}
}
