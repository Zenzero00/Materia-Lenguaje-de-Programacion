package main

import (
	"errors"
	"time"
)

type TaskManagerImpl struct {
	storage Storage
	tasks   []Task
	nextID  int
}

func NewTaskManager(storage Storage) (*TaskManagerImpl, error) {
	tm := &TaskManagerImpl{
		storage: storage,
		nextID:  1,
	}

	tasks, err := storage.Load()
	if err != nil {
		return nil, err
	}

	tm.tasks = tasks
	if len(tasks) > 0 {
		tm.nextID = tasks[len(tasks)-1].ID + 1
	}

	return tm, nil
}

func (tm *TaskManagerImpl) Add(description string) error {
	now := time.Now()
	task := Task{
		ID:          tm.nextID,
		Description: description,
		Status:      Todo,
		CreatedAt:   now,
		UpdatedAt:   now,
		CompletedAt: time.Time{},
	}

	tm.tasks = append(tm.tasks, task)
	tm.nextID++
	return tm.storage.Save(tm.tasks)
}

func (tm *TaskManagerImpl) UpdateStatus(id int, status Status) error {
	now := time.Now()
	for i := range tm.tasks {
		if tm.tasks[i].ID == id {
			oldStatus := tm.tasks[i].Status
			tm.tasks[i].Status = status
			tm.tasks[i].UpdatedAt = now

			if status == Done && oldStatus != Done {
				tm.tasks[i].CompletedAt = now
			}

			if status != Done && oldStatus == Done {
				tm.tasks[i].CompletedAt = time.Time{}
			}

			return tm.storage.Save(tm.tasks)
		}
	}
	return errors.New("tarea no encontrada")
}

func (tm *TaskManagerImpl) Delete(id int) error {
	for i, task := range tm.tasks {
		if task.ID == id {
			tm.tasks = append(tm.tasks[:i], tm.tasks[i+1:]...)
			return tm.storage.Save(tm.tasks)
		}
	}
	return errors.New("tarea no encontrada")
}

func (tm *TaskManagerImpl) ListAll() ([]Task, error) {
	return tm.tasks, nil
}

func (tm *TaskManagerImpl) ListByStatus(status Status) ([]Task, error) {
	var result []Task
	for _, task := range tm.tasks {
		if task.Status == status {
			result = append(result, task)
		}
	}
	return result, nil
}

func (tm *TaskManagerImpl) GetTaskCount() int {
	return len(tm.tasks)
}
