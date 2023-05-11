package asserters

import (
	"fmt"
	"reflect"
)

func AssertEqual(expected interface{}, actual interface{}) {
	if !reflect.DeepEqual(expected, actual) {
		fmt.Printf("Assertion failed: Expected %v, but got %v\n", expected, actual)
	}
}
