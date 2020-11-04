package io.nash.openlimits;

public class BinanceCredentials {
    public final String apiKey;
    public final String apiSecret;

    public BinanceCredentials(String apiKey, String apiSecret) {
        this.apiKey = apiKey;
        this.apiSecret = apiSecret;
    }

    @Override
    public String toString() {
        return "BinanceCredentials{" +
                "apiKey='" + apiKey + '\'' +
                ", apiSecret='" + apiSecret + '\'' +
                '}';
    }
}
