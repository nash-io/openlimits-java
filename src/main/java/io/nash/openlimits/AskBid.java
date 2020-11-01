package io.nash.openlimits;


public class AskBid {
    public final float price;
    public final float qty;

    public AskBid(float price, float qty) {
        this.price = price;
        this.qty = qty;
    }

    @Override
    public String toString() {
        return "AskBid{" +
                "price=" + price +
                ", qty=" + qty +
                '}';
    }
}
