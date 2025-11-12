package code

import (
	"math/rand"
	"time"

	"github.com/ToffaKrtek/multiapp/code-generator/types"
)

const CODE_LENGTH = 6

var expireTime = time.Second * 0

func Generate() *types.Code {
	runes := []rune{}
	for range CODE_LENGTH {
		runes = append(runes, rune('a'+rand.Intn(26)))
	}
	if expireTime != 0 {
		return types.NewCode(runes, types.SetExpireCodeFunc(expireTime))
	}
	return types.NewCode(runes)
}
