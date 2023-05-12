package asserters

import (
	"testing"
	"reflect"
)

func AssertEqual(t *testing.T, expected interface{}, actual interface{}) {
	if !reflect.DeepEqual(expected, actual) {
		t.Errorf("Assertion failed: Expected %v, but got %v\n", expected, actual)
	}
}
