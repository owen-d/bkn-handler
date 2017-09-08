.RECIPEPREFIX = >
.PHONY: deploy


HELM_NAMESPACE=bkn-handler

deploy:
> cd k8s ; \
> helm upgrade --install --namespace ${HELM_NAMESPACE} --values ./extravals.yaml bkn-handler ./bkn-handler
