package ships

import "testing"

func TestAdd(t *testing.T) {
	result := Add(2, 3)
	expected := uint(5)

	if result != expected {
		t.Errorf("Addition test failed: expected %d, got %d", expected, result)
	}
}
