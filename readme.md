# Image classification with Spin and Tensorflow

Spin application that uses a Wasm component to perform image classification using MobileNetV2.

```
$ spin build && spin up
Serving http://127.0.0.1:3000
Available Routes:
  api: http://127.0.0.1:3000/api (wildcard)
  frontend: http://127.0.0.1:3000 (wildcard)

[TS API]: Received image of size 5964. Running classification.
[Rust classifier]: Loaded Tensorflow model.
[Rust classifier]: Resized image to 224x224 px.
[Rust classifier]: Probability: 0.7306876, class: golden retriever.
[TS API]: Classification successful. Sending result golden retriever,0.7306876182556152
```
