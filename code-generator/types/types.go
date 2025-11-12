package types

import (
	"fmt"
	"time"
)

type Code struct {
	Runes      []rune
	ExpireDate time.Time
}

const TIME_EXPIRE = time.Minute * 2

type CodeFunc func(*Code)

func SetExpireCodeFunc(dur time.Duration) CodeFunc {
	return func(c *Code) {
		c.ExpireDate = time.Now().Add(dur)
	}
}

func NewCode(runes []rune, funcs ...CodeFunc) *Code {
	expireDate := time.Now().Add(TIME_EXPIRE)
	code := &Code{
		Runes:      runes,
		ExpireDate: expireDate,
	}
	for _, f := range funcs {
		f(code)
	}
	return code
}

func (c *Code) IsEqual(input []rune) (bool, error) {
	switch true {
	case time.Now().After(c.ExpireDate):
		return false, &ExpireError{c.ExpireDate}
	case len(c.Runes) != len(input):
		return false, &LengthError{len(c.Runes), len(input)}
	default:
		for i, r := range input {
			if r != c.Runes[i] {
				return false, &SymbolError{}
			}
		}
		return true, nil
	}
}

type SymbolError struct{}

func (e *SymbolError) Error() string {
	return "Wrong rune in code"
}

type LengthError struct {
	LenExp int
	LenGot int
}

func (e *LengthError) Error() string {
	return fmt.Sprintf("Code lenght must be %d, but got %d", e.LenExp, e.LenGot)
}

type ExpireError struct {
	Date time.Time
}

func (e *ExpireError) Error() string {
	return fmt.Sprintf("Code expired at %s", e.Date.Format(time.RFC1123))
}
