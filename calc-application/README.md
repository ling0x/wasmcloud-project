# Test the provider

Invoke the provider directly:

`wash call calc-calc_provider wasmcloud:calculation/calculator.call`

# Check provider health

`nats req "wasmbus.rpc.default.calc-calc_provider.health" '{}'`

`nats req "wasmbus.rpc.default.calc-nats_messaging.health" '{}'`

# Test the nats messaging capability of the component

`nats req wasmcloud.echo '{"first_number": 10, "second_number": 20}'`

The response should be:

```
12:45:13 Sending request on "wasmcloud.echo"
12:45:13 Received with rtt 1.098972ms
{"first_number": 10, "second_number": 20}
```
