package io.nash.openlimits;

public class CoinbaseConfig {
    final public boolean sandbox;
    final public CoinbaseCredentials credentials;

    public CoinbaseConfig(boolean sandbox, CoinbaseCredentials credentials) {
        this.sandbox = sandbox;
        this.credentials = credentials;
    }

    @Override
    public String toString() {
        return "CoinbaseConfig{" +
                "sandbox=" + sandbox +
                ", credentials=" + credentials +
                '}';
    }
}
