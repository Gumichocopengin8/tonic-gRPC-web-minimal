
.PHONY: proto
proto:
	mkdir -p src/proto_client && \
	protoc -I=../proto \
		--js_out=import_style=commonjs,binary:./src/proto_client \
		--grpc-web_out=import_style=typescript,mode=grpcweb:./src/proto_client \
		../proto/*.proto
