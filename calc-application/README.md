# Test the provider

Invoke the provider directly:

`wash call calc-calc_provider wasmcloud:calculation/multiply.call`

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

# Run the calculator

```
❯ nats req calculator.add '{"first_number": 10, "second_number": 20}'
14:11:25 Sending request on "calculator.add"
14:11:25 Received with rtt 834.747µs
Result: 30

~
❯ nats req calculator.divide '{"first_number": 10, "second_number": 20}'
14:11:32 Sending request on "calculator.divide"
14:11:32 Received with rtt 1.211116ms
Result: 0

~
❯ nats req calculator.multiply '{"first_number": 10, "second_number": 20}'
14:11:39 Sending request on "calculator.multiply"
14:11:39 Received with rtt 1.536605ms
Result: 200
```

# Notes:

Currently, there is a bug in the wasmCloud which has authorization error when
trying to build the messaging-nats provider directly, so I had to use
`ghcr.io/wasmcloud/messaging-nats:0.27.0` instead of build my own.
