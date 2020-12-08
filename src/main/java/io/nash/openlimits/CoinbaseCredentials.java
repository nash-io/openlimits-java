package io.nash.openlimits;

public class CoinbaseCredentials {
    public final String apiKey;
    public final String apiSecret;
    public final String passphrase;

    public CoinbaseCredentials(String apiKey, String apiSecret, String passphrase) {
        this.apiKey = apiKey;
        this.apiSecret = apiSecret;
        this.passphrase = passphrase;
    }

    @Override
    public String toString() {
        return "CoinbaseCredentials{" +
                "apiKey='" + apiKey + '\'' +
                ", apiSecret='" + apiSecret + '\'' +
                ", passphrase='" + passphrase + '\'' +
                '}';
    }
}
