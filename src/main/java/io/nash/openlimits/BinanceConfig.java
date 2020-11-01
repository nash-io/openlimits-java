package io.nash.openlimits;

public class BinanceConfig {
    final public boolean sandbox;
    final public BinanceCredentials credentials;

    public BinanceConfig(boolean sandbox, BinanceCredentials credentials) {
        this.sandbox = sandbox;
        this.credentials = credentials;
    }
}
