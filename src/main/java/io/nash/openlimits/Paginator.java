package io.nash.openlimits;

public class Paginator {
    public final long startTime;
    public final long endTime;
    public final long limit;
    public final String before;
    public final String after;

    public Paginator(long startTime, long endTime, long limit, String before, String after) {
        this.startTime = startTime;
        this.endTime = endTime;
        this.limit = limit;
        this.before = before;
        this.after = after;
    }

    @Override
    public String toString() {
        return "Paginator{" +
                "startTime=" + startTime +
                ", endTime=" + endTime +
                ", limit=" + limit +
                ", before='" + before + '\'' +
                ", after='" + after + '\'' +
                '}';
    }
}
