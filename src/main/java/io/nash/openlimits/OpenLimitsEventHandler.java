package io.nash.openlimits;

abstract public class OpenLimitsEventHandler {
    abstract public void onPing();
    abstract public void onOrderbook(OrderbookResponse orderbook);
    abstract public void onTrades(Trade[] trades);
}
