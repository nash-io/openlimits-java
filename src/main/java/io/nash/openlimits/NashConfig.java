package io.nash.openlimits;

public class NashConfig {
    public final NashCredentials credentials;
    public final long clientId;
    public final String environment;
    public final long timeout;
    public final String affiliateCode;

    public NashConfig(NashCredentials credentials, long clientId, String environment, long timeout, String affiliateCode) {
        this.credentials = credentials;
        this.clientId = clientId;
        this.environment = environment;
        this.timeout = timeout;
        this.affiliateCode = affiliateCode;
    }
    public NashConfig(NashCredentials credentials, long clientId, String environment, long timeout) {
        this.credentials = credentials;
        this.clientId = clientId;
        this.environment = environment;
        this.timeout = timeout;
        this.affiliateCode = null;
    }

    @Override
    public String toString() {
        return "NashConfig{" +
                "credentials=" + credentials +
                ", clientId=" + clientId +
                ", environment='" + environment + '\'' +
                ", timeout=" + timeout +
                ", affiliateCode='" + affiliateCode + '\'' +
                '}';
    }
}
