package service

import (
	"context"
	"time"

	"github.com/ToffaKrtek/multiapp/code-generator/internal/code"
	"github.com/ToffaKrtek/multiapp/code-generator/pb"
)

type CodeGeneratorServer struct {
	pb.UnimplementedCodeGeneratorServer
}

func (s *CodeGeneratorServer) GenerateCode(ctx context.Context, req *pb.Request) (*pb.Response, error) {
	c := code.Generate()
	return &pb.Response{
		Code:   string(c.Runes),
		Expire: c.ExpireDate.Format(time.RFC1123),
	}, nil
}
