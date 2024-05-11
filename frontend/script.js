const taskInput = document.getElementById('taskInput');
const taskList = document.getElementById('taskList');

// Function to fetch tasks from backend and display them
async function fetchTasks() {
    try {
        const response = await fetch('/tasks');
        const tasks = await response.json();
        taskList.innerHTML = '';
        tasks.forEach(task => {
            const li = document.createElement('li');
            li.innerHTML = `
                <span class="${task.completed ? 'completed' : ''}">${task.title}</span>
                <button onclick="completeTask(${task.id})">Complete</button>
                <button onclick="deleteTask(${task.id})">Delete</button>
            `;
            taskList.appendChild(li);
        });
    } catch (error) {
        console.error('Error fetching tasks:', error);
    }
}

// Function to add a new task
async function addTask() {
    const title = taskInput.value.trim();
    if (title === '') return;
    try {
        const response = await fetch('/tasks', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ title, completed: false })
        });
        if (response.ok) {
            taskInput.value = '';
            fetchTasks();
        } else {
            console.error('Failed to add task');
        }
    } catch (error) {
        console.error('Error adding task:', error);
    }
}

// Function to mark a task as completed
async function completeTask(id) {
    try {
        const response = await fetch(`/tasks/${id}`, {
            method: 'PATCH',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ completed: true })
        });
        if (response.ok) {
            fetchTasks();
        } else {
            console.error('Failed to complete task');
        }
    } catch (error) {
        console.error('Error completing task:', error);
    }
}

// Function to delete a task
async function deleteTask(id) {
    try {
        const response = await fetch(`/tasks/${id}`, {
            method: 'DELETE'
        });
        if (response.ok) {
            fetchTasks();
        } else {
            console.error('Failed to delete task');
        }
    } catch (error) {
        console.error('Error deleting task:', error);
    }
}

// Initial fetch of tasks when the page loads
fetchTasks();
