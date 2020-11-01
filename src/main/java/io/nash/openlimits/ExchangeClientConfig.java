package io.nash.openlimits;

public class ExchangeClientConfig {
    public final NashConfig nash;
    public final BinanceConfig binance;

    public ExchangeClientConfig(NashConfig nash) {
        this.nash = nash;
        this.binance = null;
    }
    public ExchangeClientConfig(BinanceConfig binance) {
        this.nash = null;
        this.binance = binance;
    }

    @Override
    public String toString() {
        return "ExchangeClientConfig{" +
                "nash=" + nash +
                ", binance=" + binance +
                '}';
    }
}
