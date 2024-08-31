{{/* Generate object name */}}
{{- define "mychart.name" }}
{{- regexMatch "^v[0-9]+\\.[0-9]+\\.[0-9]+$" .Values.image.tag | ternary "prod" (regexReplaceAll "\\W+" .Values.image.tag "_") -}}
{{- end }}

{{/* Generate labels */}}
{{- define "mychart.labels" -}}
{{- with .Values.labels }}
{{- range $key, $value := . }}
  {{ $key }}: {{ tpl $value $ | quote }}
{{- end }}
{{- end }}
{{- end }}

 {{/* Generate annotations */}}
{{- define "mychart.annotations" -}}
{{- with .Values.annotations}}
{{- range $key, $value := . }}
  {{ $key }}: {{ tpl $value $ | quote }}
{{- end }}
  rollme: {{ randAlphaNum 5 | quote }}
{{- end }}
{{- end }}

{{/* Generate container env & envSecrets */}}
{{- define "mychart.env" -}}
{{- with .Values.env }}
{{- range $key, $value := . }}
    - name: {{ $key }}
      value: {{ tpl $value $ | quote }}
{{- end }}
{{- end }}
{{- with .Values.envSecrets}}
{{- range $key, $value := . }}
    - name: {{ $key }}
      valueFrom:
        secretKeyRef:
          name: {{ tpl $value.secretName $ | quote }}
          key: {{ tpl $value.secretKey $ | quote }}
{{- end }}
{{- end }}
{{- end }}