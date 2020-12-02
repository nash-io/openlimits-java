# OpenLimits Java Wrapper

Starting point for Openlimits wrapper in Java using rust-jni for java-python bindings.

## Build the java package

Make sure you have rust installed on your system.

To build the library please run `./gradlew assemble`.

You will have the .jar file and shared library (.so, dylic, -dll) in the build/libs folder, which can be used in other projects.

## Example usage

```
package example;

import io.nash.openlimits.*;

public class Example {
    static {
        System.loadLibrary("openlimits_java");
    }

    public static void main(String[] args) {
        String apiKey = System.getenv("NASH_API_KEY");
        String secret = System.getenv("NASH_API_SECRET");
        NashConfig nashConfig = new NashConfig(
                new NashCredentials(
                        secret,
                        apiKey
                ),
                0,
                "sandbox",
                10000
        );
        ExchangeClient client = new ExchangeClient(new ExchangeClientConfig(nashConfig));
        client.setSubscriptionCallback(new OpenLimitsEventHandler() {
            @Override
            public void onOrderbook(OrderbookResponse orderbook) {
                System.out.println(orderbook);
            }
            @Override
            public void onError() {
                System.out.println("Disconnected. Restarting bot.");
            }
        });
        client.subscribe(Subscription.orderbook("btc_usdc", 5));
    }
}
```
