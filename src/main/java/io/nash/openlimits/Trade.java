package io.nash.openlimits;

public class Trade {
    public final String id;
    // nullable
    public final String buyerOrderId;
    // nullable
    public final String sellerOrderId;
    public final String marketPair;
    public final String price;
    public final String qty;
    // nullable
    public final String fees;
    public final String side;
    // nullable
    public final String liquidity;
    public final long createdAt;

    public Trade(String id, String buyerOrderId, String sellerOrderId, String marketPair, String price, String qty, String fees, String side, String liquidity, long createdAt) {
        this.id = id;
        this.buyerOrderId = buyerOrderId;
        this.sellerOrderId = sellerOrderId;
        this.marketPair = marketPair;
        this.price = price;
        this.qty = qty;
        this.fees = fees;
        this.side = side;
        this.liquidity = liquidity;
        this.createdAt = createdAt;
    }

    @Override
    public String toString() {
        return "Trade{" +
                "id='" + id + '\'' +
                ", buyerOrderId='" + buyerOrderId + '\'' +
                ", sellerOrderId='" + sellerOrderId + '\'' +
                ", marketPair='" + marketPair + '\'' +
                ", price=" + price +
                ", qty=" + qty +
                ", fees=" + fees +
                ", side='" + side + '\'' +
                ", liquidity='" + liquidity + '\'' +
                ", createdAt=" + createdAt +
                '}';
    }
}
