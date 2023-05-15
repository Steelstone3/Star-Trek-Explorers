package presenters

import (
	ui "github.com/gizak/termui/v3"
	"github.com/gizak/termui/v3/widgets"
	"log"
)

func StartUi() {
	if err := ui.Init(); err != nil {
		log.Fatalf("failed to initialize termui: %v", err)
	}
	defer ui.Close()
}

func DisplayTable(title string, rows [][]string) {
	if err := ui.Init(); err != nil {
		log.Fatalf("failed to initialize termui: %v", err)
	}
	defer ui.Close()

	table := widgets.NewTable()
	table.Title = title
	table.Rows = rows
	table.TextStyle = ui.NewStyle(ui.ColorWhite)
	table.TextAlignment = ui.AlignCenter
	table.RowSeparator = true
	table.SetRect(0, 0, 50, 25)
	columnWidths := calculateColumnWidths(table.Rows)
	table.ColumnWidths = columnWidths

	ui.Render(table)

	uiEvents := ui.PollEvents()
	for {
		e := <-uiEvents
		switch e.ID {
		case "q", "<C-c>":
			return
		}
	}
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

func calculateColumnWidths(rows [][]string) []int {
	columnCount := len(rows[0])
	columnWidths := make([]int, columnCount)

	maximumLengthInEachColumn(rows, columnWidths)
	addPadding(columnWidths)

	return columnWidths
}

func maximumLengthInEachColumn(rows [][]string, columnWidths []int) {
	for _, row := range rows {
		for columnIndex, cell := range row {
			cellLength := len(cell)
			if cellLength > columnWidths[columnIndex] {
				columnWidths[columnIndex] = cellLength
			}
		}
	}
}

func addPadding(columnWidths []int) {
	padding := 2
	for i := range columnWidths {
		columnWidths[i] += padding
	}
}