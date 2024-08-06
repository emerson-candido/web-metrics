# Web Metrics Helm Chart

This Helm Chart is designed to provision the application `web_metrics` on Kubernetes.

## Values

| Value                                      | Description                                                           | Default Value                  |
|--------------------------------------------|-----------------------------------------------------------------------|--------------------------------|
| `endpoints`                                | List of urls to collect metrics                                       | `Null`                         |
| `envs`                                     | List of environment variables to be set                               | `Null`                         |
| `image`                                    | Image path of the application                                         | `ecandido/web-metrics:tagname` |
| `imagePullPolicy`                          | Default behavior to pull images                                       | `IfNotPresent`                 |
| `livenessProbe.httpGet.path`               | Path to perform health check                                          | `/health`                      |
| `livenessProbe.httpGet.port`               | Port to perform the health check                                      | `9898`                         |
| `livenessProbe.initialDelaySeconds`        | Time in seconds to wait before perform the check                      | `30`                           |
| `livenessProbe.timeoutSeconds`             | Time in seconds to wait before perform a failure                      | `10`                           |
| `port.name`                                | Port name of the Kubernetes `service` to run                          | `http`                         |
| `port.number`                              | Port number to listen the Kubernetes `service` and to `containerPort` | `9898`                         |
| `readinessProbe.httpGet.path`              | Path to perform health check                                          | `/health`                      |
| `readinessProbe.httpGet.port`              | Port to perform the health check                                      | `9898`                         |
| `readinessProbe.initialDelaySeconds`       | Time in seconds to wait before perform the check                      | `30`                           |
| `readinessProbe.timeoutSeconds`            | Time in seconds to wait before perform a failure                      | `10`                           |
| `replicas`                                 | Number of replicas to run on the Kubernetes `deployment`              | `1`                            |
| `retry`                                    | Waiting time in seconds to perform the health check again             | `30`                           |
| `resources.limits.memory`                  | Memory limit allocated to the `pod`                                   | `32Mi`                         |
| `resources.requests.cpu`                   | CPU requested to the `pod`                                            | `50m`                          |
| `resources.requests.memory`                | Memory requested to the `pod`                                         | `32Mi`                         |
| `rollingUpdate.maxSurge`                   | Number of additional pods during `rollout` process                    | `1`                            |
| `rollingUpdate.maxUnavailable`             | Number of unavailable pods during `rollout` process                   | `0`                            |
| `securityContext.allowPrivilegeEscalation` | Allow or not user to perform privilege escalation activities          | `false`                        |
| `securityContext.fsGroup`                  | UserID to perform file system operations                              | `1000`                         |
| `securityContextrunAsGroup`                | GroupID allowed to run in the `pod`                                   | `1000`                         |
| `securityContextrunAsNonRoot`              | Allow only `non-root` users to run in the `pod`                       | `true`                         |
| `securityContextrunAsUser`                 | GroupID allowed to run in the `pod`                                   | `1000`                         |
| `service.protocol`                         | Protocol to listen on Kubernetes `service`                            | `TCP`                          |
| `service.Type`                             | Type of Kubernetes `service`                                          | `ClusterIP`                    |
| `settingsFile.name`                        | Name of the settings file                                             | `settings.yaml`                |
| `settingsFile.path`                        | Path that hosts the settings file                                     | `/tpm`                         |

