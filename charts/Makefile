_PHONY: install-http install-mtls uninstall log-openvasd
TLS_PATH="../rust/examples/tls/self-signed"

install-http:
	helm install --namespace openvasd --create-namespace openvasd openvasd/ --values openvasd/values.yaml --values openvasd/http-root.yaml

install-mtls:
	helm install --namespace openvasd --create-namespace openvasd openvasd/ --values openvasd/values.yaml

uninstall:
	helm uninstall -n openvasd openvasd

log-openvasd:
	kubectl logs -n openvasd deployment/openvasd -c openvasd
