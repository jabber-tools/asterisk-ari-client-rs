# examples

Examples in this folder can be run against dockerized version of asterisk that can be found in [ast-docker](./ast-docker) folder. Configuration files copied into image (e.g. extensions.conf) where tested with Asterisk 20.5.

Build asterisk image:
```
cd ast-docker
docker build -t asterisk .  
```

Then run the image in interactive mode and start asterisk in verbose CLI mode:
```
docker run -it -p 8088:8088/tcp -p 5060:5060/tcp -p 5060:5060/udp -p 16384-16394:16384-16394 asterisk /bin/bash
asterisk -cgvvvvvvvvvvvvvvv
```

In your soft phone (e.g. Zoiper5 or MicroSIP) configure user **6001@localhost:5060** and dial extension 100.

**NOTE:** asterisk config in folder **asterisk_docker** is based on [Asterisk getting started page](https://docs.asterisk.org/Getting-Started/Hello-World) with ARI setup added.
For additional info about ARI see [here](https://docs.asterisk.org/Configuration/Interfaces/Asterisk-REST-Interface-ARI/Getting-Started-with-ARI).


Once your asterisk is up & running you can run full example:

```
cargo run --example simple_client
```

or specific examples (currently recording control example):

```
cargo run --example recording
```

To connect to ARI websocket directly without simple_client above you can also use **wscat**, details [here](https://docs.asterisk.org/Configuration/Interfaces/Asterisk-REST-Interface-ARI/Getting-Started-with-ARI/#configuring-asterisk).

```
wscat -c "ws://localhost:8088/ari/events?api_key=asterisk:asterisk&app=my-ast-app"
```