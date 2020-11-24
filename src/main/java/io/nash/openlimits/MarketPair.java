package io.nash.openlimits;

public class MarketPair {
    public final String base;
    public final String quote;
    public final String symbol;
    public final String baseIncrement;
    public final String quoteIncrement;
    public final String minBaseTradeSize;
    public final String minQuoteTradeSize;

    public MarketPair(String base, String quote, String symbol, String baseIncrement, String quoteIncrement, String minBaseTradeSize, String minQuoteTradeSize) {
        this.base = base;
        this.quote = quote;
        this.symbol = symbol;
        this.baseIncrement = baseIncrement;
        this.quoteIncrement = quoteIncrement;
        this.minBaseTradeSize = minBaseTradeSize;
        this.minQuoteTradeSize = minQuoteTradeSize;
    }

    @Override
    public String toString() {
        return "MarketPair{" +
                "base='" + base + '\'' +
                ", quote='" + quote + '\'' +
                ", symbol='" + symbol + '\'' +
                ", baseIncrement='" + baseIncrement + '\'' +
                ", quoteIncrement='" + quoteIncrement + '\'' +
                ", minBaseTradeSize='" + minBaseTradeSize + '\'' +
                ", minQuoteTradeSize='" + minQuoteTradeSize + '\'' +
                '}';
    }
}
