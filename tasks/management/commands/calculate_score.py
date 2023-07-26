from django.core.management.base import BaseCommand
from django.utils import timezone

from tasks.services import calculate_score


class Command(BaseCommand):
    help = 'Do the calculation of the score of the tasks'

    def handle(self, *args, **kwargs):
        calculate_score()
