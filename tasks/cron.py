from django_cron import CronJobBase, Schedule

from tasks.services import calculate_score


class CronJob(CronJobBase):
    RUN_EVERY_MINS = 60  # every hour

    schedule = Schedule(run_every_mins=RUN_EVERY_MINS)
    code = 'tasks.cron.CronJob'  # a unique code

    def do(self):
        calculate_score()
