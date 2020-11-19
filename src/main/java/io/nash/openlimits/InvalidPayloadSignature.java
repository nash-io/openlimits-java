package io.nash.openlimits;

public class InvalidPayloadSignature extends OpenLimitsException {
    public InvalidPayloadSignature(String s) {
        super(s);
    }
}
