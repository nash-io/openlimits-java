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

    public static void main(String[] args) {
        String apiKey = System.getenv("NASH_API_SESSION_PROD");
        String secret = System.getenv("NASH_API_SECRET_PROD");
        NashConfig config = new NashConfig(
                new NashCredentials(secret, apiKey),
                0,
                "production",
                1000
        );
        final ExchangeClient client = new ExchangeClient(new ExchangeClientConfig(config));


        client.subscribeTrades("btc_usdc", (TradesResponse trades) -> {
            System.out.println(trades);
        });

        client.subscribeError(err -> {
            System.out.println("Experienced an error, cleaning up");
            client.cancelAllOrders(new CancelAllOrdersRequest("btc_usdc"));
            client.cancelAllOrders(new CancelAllOrdersRequest("noia_usdc"));
            client.disconnect();
        });

        client.subscribeDisconnect(() -> {
            System.out.println("Resetting bot");
        });
    }
}
```
