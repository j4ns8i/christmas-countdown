REGISTRY := us-central1-docker.pkg.dev
IMAGE    := christmas-countdown-372221/christmas-countdown/christmas-countdown
TAG      := latest
FQ_IMAGE := $(REGISTRY)/$(IMAGE):$(TAG)

.PHONY: docker-build
docker-build:
	docker build -t $(FQ_IMAGE) .

.PHONY: docker-push
docker-push: docker-build
	docker push $(FQ_IMAGE)
