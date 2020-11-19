package io.nash.openlimits;

public class Balance {
    public final String asset;
    public final String total;
    public final String free;

    public Balance(String asset, String total, String free) {
        this.asset = asset;
        this.total = total;
        this.free = free;
    }

    @Override
    public String toString() {
        return "Balance{" +
                "asset='" + asset + '\'' +
                ", total='" + total + '\'' +
                ", free='" + free + '\'' +
                '}';
    }
}
