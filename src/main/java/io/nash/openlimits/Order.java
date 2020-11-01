package io.nash.openlimits;

public class Order {
    public final String id;
    public final String market;
    public final String clientOrderId;
    public final long createdAt;
    public final String orderType;
    public final String side;
    public final String status;
    public final String size;
    public final String price;

    public Order(String id, String market, String clientOrderId, long createdAt, String orderType, String side, String status, String size, String price) {
        this.id = id;
        this.market = market;
        this.clientOrderId = clientOrderId;
        this.createdAt = createdAt;
        this.orderType = orderType;
        this.side = side;
        this.status = status;
        this.size = size;
        this.price = price;
    }

    @Override
    public String toString() {
        return "Order{" +
                "id='" + id + '\'' +
                ", market='" + market + '\'' +
                ", clientOrderId='" + clientOrderId + '\'' +
                ", createdAt=" + createdAt +
                ", orderType='" + orderType + '\'' +
                ", side='" + side + '\'' +
                ", status='" + status + '\'' +
                ", size='" + size + '\'' +
                ", price='" + price + '\'' +
                '}';
    }
}
