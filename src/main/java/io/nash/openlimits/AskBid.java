package io.nash.openlimits;


public class AskBid {
    public final String price;
    public final String qty;

    public AskBid(String price, String qty) {
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
