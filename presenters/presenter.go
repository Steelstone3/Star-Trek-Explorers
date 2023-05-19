package presenters

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Print(message string) {
	fmt.Println(message)
}

func GetStringFromConsole(prompt string) string {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print(prompt)
	text, _ := reader.ReadString('\n')
	text = strings.TrimSpace(text)
	return text
}

func GetUintFromConsole(prompt string) uint {
	var text = GetStringFromConsole(prompt)
	num, err := strconv.ParseUint(text, 10, 64)
	if err != nil {
		return 0
	}
	return uint(num)
}
