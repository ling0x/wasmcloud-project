# wasmCloud

https://wasmcloud.com/docs/

## Start

Build the component: `wash build`

Start up the host and NATS etc: `wash up -d`

Deploy the component: `wash app deploy wadm.yaml`

## Start with different NATS socket

`wash up --nats-websocket-port 4444 # defaults to 4223`

## Debug

`wash spy --experimental hello_world-http_component`

## Undeploy

Remove all apps, providers and links from the host: `wash app undeploy --all`

## Cleanup

When you're done with the application, delete it from the wasmCloud environment:
`wash app delete hello-world`

Shut down the environment: `wash down --all`

## Component

`wash new component hello --template-name hello-world-rust`

`wash dev`
