class TaskService:
    def complete(self, task):
        """
        Complete the task
        """
        task.completed = task.updated
        task.save()

    def uncomplete(self, task):
        """
        Uncomplete the task
        """
        task.completed = None
        task.save()
