package io.nash.openlimits;

public class Candle {
    public final long time;
    public final String low;
    public final String high;
    public final String open;
    public final String close;
    public final String volume;

    public Candle(long time, String low, String high, String open, String close, String volume) {
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
