package io.nash.openlimits;

public class Ticker {
    public final String price;

    public Ticker(String price) {
        this.price = price;
    }

    @Override
    public String toString() {
        return "Ticker{" +
                "price=" + price +
                '}';
    }
}
