package io.nash.openlimits;

public class ExchangeClientConfig {
    public final NashConfig nash;
    public final BinanceConfig binance;
    public final CoinbaseConfig coinbase;

    public ExchangeClientConfig(NashConfig nash) {
        this.nash = nash;
        this.binance = null;
        this.coinbase = null;
    }
    public ExchangeClientConfig(BinanceConfig binance) {
        this.nash = null;
        this.binance = binance;
        this.coinbase = null;
    }
    public ExchangeClientConfig(CoinbaseConfig coinbase) {
        this.nash = null;
        this.binance = null;
        this.coinbase = coinbase;
    }

    @Override
    public String toString() {
        return "ExchangeClientConfig{" +
                "nash=" + nash +
                ", binance=" + binance +
                ", coinbase=" + coinbase +
                '}';
    }
}
