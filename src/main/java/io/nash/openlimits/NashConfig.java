package io.nash.openlimits;

public class NashConfig {
    public final NashCredentials credentials;
    public final long clientId;
    public final String environment;
    public final long timeout;

    public NashConfig(NashCredentials credentials, long clientId, String environment, long timeout) {
        this.credentials = credentials;
        this.clientId = clientId;
        this.environment = environment;
        this.timeout = timeout;
    }

    @Override
    public String toString() {
        return "NashConfig{" +
                "credentials=" + credentials +
                ", clientId=" + clientId +
                ", environment='" + environment + '\'' +
                ", timeout=" + timeout +
                '}';
    }
}
