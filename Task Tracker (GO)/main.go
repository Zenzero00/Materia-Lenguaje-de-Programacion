package main

import "fmt"

func main() {
	storage := NewJSONStorage("tasks.json")

	taskManager, err := NewTaskManager(storage)
	if err != nil {
		fmt.Println("Error al inicializar el gestor de tareas:", err)
		return
	}

	ui := NewUI(taskManager)
	ui.Run()
}
