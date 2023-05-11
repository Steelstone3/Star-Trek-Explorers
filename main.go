package main

import (
    "fmt"
    "github.com/Steelstone3/Star-Trek-Explorers/controllers"
)

func main() {
    result := controllers.Add(2, 3)
    fmt.Println("Result:", result)
}
