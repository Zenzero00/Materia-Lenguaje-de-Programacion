package main

import (
	"fmt"
	"strings"
)

type UI struct {
	taskManager *TaskManagerImpl
}

func NewUI(taskManager *TaskManagerImpl) *UI {
	return &UI{taskManager: taskManager}
}

func (ui *UI) ShowMenu() {
	fmt.Println("\n=== GESTOR DE TAREAS ===")
	fmt.Println("1. Ver todas las tareas")
	fmt.Println("2. Agregar nueva tarea")
	fmt.Println("3. Cambiar estado de tarea")
	fmt.Println("4. Eliminar tarea")
	fmt.Println("5. Ver tareas por estado")
	fmt.Println("6. Salir")
	fmt.Print("Elige una opción: ")
}

func (ui *UI) PrintTasks(tasks []Task) {
	if len(tasks) == 0 {
		fmt.Println("No hay tareas para mostrar")
		return
	}

	maxIDWidth := 2
	maxDescWidth := len("Descripción")
	maxStatusWidth := len("Estado")

	for _, task := range tasks {
		if len(fmt.Sprintf("%d", task.ID)) > maxIDWidth {
			maxIDWidth = len(fmt.Sprintf("%d", task.ID))
		}
		if len(task.Description) > maxDescWidth {
			maxDescWidth = len(task.Description)
		}
		if len(string(task.Status)) > maxStatusWidth {
			maxStatusWidth = len(string(task.Status))
		}
	}

	maxIDWidth += 2
	maxDescWidth += 2
	maxStatusWidth += 2

	fmt.Println()
	fmt.Printf("┌%s┬%s┬%s┐\n",
		strings.Repeat("─", maxIDWidth),
		strings.Repeat("─", maxDescWidth),
		strings.Repeat("─", maxStatusWidth))

	fmt.Printf("│%-*s│%-*s│%-*s│\n",
		maxIDWidth, "ID",
		maxDescWidth, "Descripción",
		maxStatusWidth, "Estado")

	fmt.Printf("├%s┼%s┼%s┤\n",
		strings.Repeat("─", maxIDWidth),
		strings.Repeat("─", maxDescWidth),
		strings.Repeat("─", maxStatusWidth))

	for _, task := range tasks {
		fmt.Printf("│%-*d│%-*s│%-*s│\n",
			maxIDWidth, task.ID,
			maxDescWidth, task.Description,
			maxStatusWidth, task.Status)
	}

	fmt.Printf("└%s┴%s┴%s┘\n",
		strings.Repeat("─", maxIDWidth),
		strings.Repeat("─", maxDescWidth),
		strings.Repeat("─", maxStatusWidth))
}

func (ui *UI) HandleListAll() {
	tasks, err := ui.taskManager.ListAll()
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	ui.PrintTasks(tasks)
}

func (ui *UI) HandleAddTask() {
	fmt.Print("Descripción de la tarea: ")
	var description string
	fmt.Scanln()
	fmt.Scanln(&description)

	if err := ui.taskManager.Add(description); err != nil {
		fmt.Println("Error al agregar tarea:", err)
	} else {
		fmt.Println("Tarea agregada exitosamente")
	}
}

func (ui *UI) HandleUpdateStatus() {
	ui.HandleListAll()
	fmt.Print("ID de la tarea a actualizar: ")
	var id int
	fmt.Scan(&id)

	fmt.Println("Estados disponibles:")
	fmt.Println("1. Por hacer")
	fmt.Println("2. En progreso")
	fmt.Println("3. Terminada")
	fmt.Print("Nuevo estado (1-3): ")

	var choice int
	fmt.Scan(&choice)

	var status Status
	switch choice {
	case 1:
		status = Todo
	case 2:
		status = InProgress
	case 3:
		status = Done
	default:
		fmt.Println("Opción no válida")
		return
	}

	if err := ui.taskManager.UpdateStatus(id, status); err != nil {
		fmt.Println("Error:", err)
	} else {
		fmt.Println("Estado actualizado exitosamente")
	}
}

func (ui *UI) HandleDelete() {
	ui.HandleListAll()
	fmt.Print("ID de la tarea a eliminar: ")
	var id int
	fmt.Scan(&id)

	if err := ui.taskManager.Delete(id); err != nil {
		fmt.Println("Error:", err)
	} else {
		fmt.Println("Tarea eliminada exitosamente")
	}
}

func (ui *UI) HandleListByStatus() {
	fmt.Println("Estados disponibles:")
	fmt.Println("1. Por hacer")
	fmt.Println("2. En progreso")
	fmt.Println("3. Terminada")
	fmt.Print("Estado a listar (1-3): ")

	var choice int
	fmt.Scan(&choice)

	var status Status
	switch choice {
	case 1:
		status = Todo
	case 2:
		status = InProgress
	case 3:
		status = Done
	default:
		fmt.Println("Opción no válida")
		return
	}

	tasks, err := ui.taskManager.ListByStatus(status)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	ui.PrintTasks(tasks)
}

func (ui *UI) Run() {
	fmt.Printf("Tareas cargadas: %d\n", ui.taskManager.GetTaskCount())

	for {
		ui.ShowMenu()
		var choice int
		fmt.Scan(&choice)

		switch choice {
		case 1:
			ui.HandleListAll()
		case 2:
			ui.HandleAddTask()
		case 3:
			ui.HandleUpdateStatus()
		case 4:
			ui.HandleDelete()
		case 5:
			ui.HandleListByStatus()
		case 6:
			fmt.Println("¡Hasta luego!")
			return
		default:
			fmt.Println("Opción no válida")
		}
	}
}
