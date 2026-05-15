from dsa_kuuking.burmese_native import PriorityQueue

class Task:
    """
    A task, a todo that has a name and priority. It is used for the 'Queue'.

    Example:
    >>> watch_tv = Task(3, "Watch TV")
    >>> print(watch_tv)
    """
    def __init__(self, priority, name):
        self.name = name
        self.priority = priority

    def __str__(self):
        return f"Task(name={self.name}, priority={self.priority})"
    
class Queue:
    def __init__(self):
        self.tasks = PriorityQueue()
        # TODO: Temporary fix
        self.size = 0

    def add_task(self, task):
        self.tasks.push(task.priority, task.name)
        self.size += 1

    def get_next_task(self):
        if self.size > 0:
            self.size -= 1
            return self.tasks.pop()
        else:
            return None
    
if __name__ == "__main__":
    task_eat_dinner = Task(2, "Eat Dinner")
    task_do_homework = Task(1, "Do Homework")

    task_watch_tv = Task(3, "Watch PrimeTimeAgen - Rust")
    pq = Queue()
    pq.add_task(task_eat_dinner)
    pq.add_task(task_do_homework)
    pq.add_task(task_watch_tv)
    print("Tasks in priority order:")
    while True:
        next_task = pq.get_next_task()
        if next_task is None:
            break
        print(next_task)
    