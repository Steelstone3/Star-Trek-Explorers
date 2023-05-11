package main

import (
    "fmt"
    "github.com/Steelstone3/Shhh-Library/controllers"
)

func main() {
    result := controllers.Add(2, 3)
    fmt.Println("Result:", result)
}
