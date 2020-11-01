package io.nash.openlimits;

public class Candle {
    public final long time;
    public final float low;
    public final float high;
    public final float open;
    public final float close;
    public final float volume;

    public Candle(long time, float low, float high, float open, float close, float volume) {
        this.time = time;
        this.low = low;
        this.high = high;
        this.open = open;
        this.close = close;
        this.volume = volume;
    }

    @Override
    public String toString() {
        return "Candle{" +
                "time=" + time +
                ", low=" + low +
                ", high=" + high +
                ", open=" + open +
                ", close=" + close +
                ", volume=" + volume +
                '}';
    }
}
