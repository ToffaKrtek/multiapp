package main

import (
	"context"
	"time"

	"github.com/ToffaKrtek/multiapp/code-generator/internal/code"
	"go-micro.dev/v5"
)

type Request struct{}

type Response struct {
	Code   string `json:"code"`
	Expire string `json:"expire"`
}

type Generate struct{}

func (g *Generate) Code(ctx context.Context, req *Request, resp *Response) error {
	code := code.Generate()
	resp.Code = string(code.Runes)
	resp.Expire = code.ExpireDate.Format(time.RFC1123)
	return nil
}

func main() {
	service := micro.NewService(
		micro.Name("code-generator"),
		micro.Address(":8080"),
	)
	service.Init()
	service.Handle(new(Generate))
	service.Run()
}
