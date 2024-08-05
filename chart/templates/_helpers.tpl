{{- define "default.name" }}
{{- .Release.Name | trunc 63 | trimSuffix "-" | replace "_" "-" | quote -}}
{{- end }}

{{- define  "default.labels" }}
{{ printf "app: %s" .Release.Name | trunc 63 | trimSuffix "-" | replace "_" "-"  }}
{{ printf "app_version: %s" .Chart.AppVersion  }}
{{ printf "chart_version: %s" .Chart.Version  }}
{{- end }}
