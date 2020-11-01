package io.nash.openlimits;

public class Ticker {
    public final float price;

    public Ticker(float price) {
        this.price = price;
    }

    @Override
    public String toString() {
        return "Ticker{" +
                "price=" + price +
                '}';
    }
}
