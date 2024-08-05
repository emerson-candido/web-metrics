# Web Metrics

The `web_metrics` application perform http requests to a pre-defined urls list, collecting key information, and then provide `Prometheus` metrics.

## Overview

The application collect the `status code` of the request, besides the `response_time`, and then report those values in `prometheus metrics` format.



## Settings File

The application requires a `yaml` settings file, that provides the information:

| Settings    | Description                                                                            |
|-------------|----------------------------------------------------------------------------------------|
| `endpoints` | List of `urls` to get metrics                                                          |
| `port`      | Port that application is listening                                                     |
| `retry`     | Waiting time to perform the request against list of endpoints, and then update metrics |


## Environment variables

| Environment Variable  | Description               | Default Value   |
|-----------------------|---------------------------|-----------------|
| `SETTINGS_FILEPATH`   | Path of the settings file | `settings.yaml` |


## Sample Output

During the execution of the application, by the command `make run` you can see the logs related the list of urls that application is getting metrics.

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/web_metrics`
[2024-08-05T13:45:34Z INFO  web_metrics] Update metrics for https://github.com: status_code = 200, response_time = 70.536149ms
[2024-08-05T13:45:44Z INFO  web_metrics] Update metrics for https://github.com: status_code = 200, response_time = 72.33437ms
```

The metrics are available in the endpoint `http://localhost:9898/metrics`, according to:

```
$ curl http://localhost:9898/metrics                                                                                                                                      ✔ 
# HELP http_request_duration_seconds HTTP request duration in seconds
# TYPE http_request_duration_seconds gauge
http_request_duration_seconds{endpoint="https://github.com"} 0.062149788
# HELP http_response_status_codes_total HTTP response status codes
# TYPE http_response_status_codes_total counter
http_response_status_codes_total{endpoint="https://github.com",status="200"} 7
```

## Kubernetes Deployment

The application can be deployed in the Kubernetes via the [helm chart](chart/README.md),
