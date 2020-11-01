package io.nash.openlimits;

public class NashCredentials {
    public final String secret;
    public final String session;

    public NashCredentials(String secret, String session) {
        this.secret = secret;
        this.session = session;
    }

    @Override
    public String toString() {
        return "NashCredentials{" +
                "secret='" + secret + '\'' +
                ", session='" + session + '\'' +
                '}';
    }
}
