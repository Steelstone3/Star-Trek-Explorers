package presenters

import (
	"log"
	ui "github.com/gizak/termui/v3"
	"github.com/gizak/termui/v3/widgets"
)

func StartUi() {
	if err := ui.Init(); err != nil {
		log.Fatalf("failed to initialize termui: %v", err)
	}
	defer ui.Close()
}

func DisplayTable(rows []string, columns []string) {
	
}

func DisplayList(title string, options []string) string {
	list := widgets.NewList()
	list.Title = title
	list.Rows = options
	list.WrapText = true
	list.SetRect(0, 0, 25, 8)

	ui.Render(list)

	return ""
}