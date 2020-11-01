package io.nash.openlimits;

public class Trade {
    public final String id;
    public final String orderId;
    public final String marketPair;
    public final float price;
    public final float qty;
    // nullable
    public final float fees;
    public final String side;
    // nullable
    public final String liquidity;
    public final long createdAt;

    public Trade(String id, String orderId, String marketPair, float price, float qty, float fees, String side, String liquidity, long createdAt) {
        this.id = id;
        this.orderId = orderId;
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
                ", order_id='" + orderId + '\'' +
                ", market_pair='" + marketPair + '\'' +
                ", price=" + price +
                ", qty=" + qty +
                ", fees=" + fees +
                ", side='" + side + '\'' +
                ", liquidity='" + liquidity + '\'' +
                ", createdAt=" + createdAt +
                '}';
    }
}
