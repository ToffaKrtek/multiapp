package code

import (
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestGenerate(t *testing.T) {

	code := Generate()
	code1 := Generate()
	eq, err := code.IsEqual(code1.Runes)
	assert.Error(t, err)
	assert.False(t, eq)

	if CODE_LENGTH > 1 {
		eq, err = code.IsEqual(code.Runes[:CODE_LENGTH-2])
		assert.Error(t, err)
		assert.False(t, eq)
	}

	eq, err = code.IsEqual(code.Runes)
	assert.NoError(t, err)
	assert.True(t, eq)
}

func TestGenerateExpire(t *testing.T) {

	expireTime = time.Second * 1
	code := Generate()
	time.Sleep(expireTime)
	eq, err := code.IsEqual(code.Runes)
	assert.Error(t, err)
	assert.False(t, eq)
}
