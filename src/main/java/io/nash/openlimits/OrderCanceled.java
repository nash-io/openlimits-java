package io.nash.openlimits;

public class OrderCanceled {
    public final String id;
    public OrderCanceled(String id) {
        this.id = id;
    }

    @Override
    public String toString() {
        return "OrderCanceled{" +
                "id='" + id + '\'' +
                '}';
    }
}
