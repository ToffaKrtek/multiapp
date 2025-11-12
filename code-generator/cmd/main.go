package main

import (
	"log"
	"net"

	"github.com/ToffaKrtek/multiapp/code-generator/internal/service"
	"github.com/ToffaKrtek/multiapp/code-generator/pb"
	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection"
)

func main() {
	lis, err := net.Listen("tcp", ":8080")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()

	reflection.Register(grpcServer)

	pb.RegisterCodeGeneratorServer(grpcServer, &service.CodeGeneratorServer{})

	log.Println("gRPC server running on :8080")
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
