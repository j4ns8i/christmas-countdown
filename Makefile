REGISTRY                 := us-central1-docker.pkg.dev
IMAGE_NAME               := christmas-countdown-372221/christmas-countdown/christmas-countdown
IMAGE                    := $(REGISTRY)/$(IMAGE_NAME)
TAG                      := v0.1.0
TAG_DEBUG                := $(TAG)-debug
IMAGE_TAG_RELEASE        := $(IMAGE):$(TAG)
IMAGE_TAG_DEBUG          := $(IMAGE):$(TAG_DEBUG)
IMAGE_TAG_LATEST_RELEASE := $(IMAGE):latest
IMAGE_TAG_LATEST_DEBUG   := $(IMAGE):latest-debug

.PHONY: docker-build
docker-build:
	DOCKER_BUILDKIT=1 docker build \
					-t $(IMAGE_TAG_RELEASE) \
					--build-arg CC_CARGO_BUILD_FLAGS="--release"\
					--build-arg CC_CARGO_INSTALL_FLAGS="" \
					.
	docker tag $(IMAGE_TAG_RELEASE) $(IMAGE_TAG_LATEST_RELEASE)
	DOCKER_BUILDKIT=1 docker build \
					-t $(IMAGE_TAG_DEBUG) \
					.
	docker tag $(IMAGE_TAG_DEBUG) $(IMAGE_TAG_LATEST_DEBUG)

.PHONY: docker-push
docker-push: docker-build
	docker push $(IMAGE_TAG_RELEASE)
	docker push $(IMAGE_TAG_LATEST_RELEASE)
	# docker push $(IMAGE_TAG_DEBUG)
	# docker push $(IMAGE_TAG_LATEST_DEBUG)

.PHONY: dev-up
dev-up: docker-build
	docker run \
		-p 8080:8080 \
		-ti \
		--rm \
		--name christmas-countdown-dev \
		$(IMAGE_TAG_DEBUG)
