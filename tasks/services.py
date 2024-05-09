from django.contrib.auth.models import User
from django.utils import timezone

from tasks.models import Task
from users.models import UserData

DAY_HOURS = 24


def calculate_score_for_user(user_data: UserData, now):
    """
    :param user_data: User object, which score needs to be calculated
    """
    user_data.urgency_calculated_at = now
    user_data.save()

    tasks = Task.objects.get_active(user=user_data.user)
    for task in tasks:
        task.calculate_urgency(now)
        task.save()


def calculate_score(now=None):
    if now is None:
        now = timezone.now()

    users = User.objects.all()
    for user in users:
        user_data = UserData.objects.filter(user=user).first()
        # Check how many hours have passed since last calculation
        needs_calculation = False
        if user_data.urgency_calculated_at is None:
            needs_calculation = True
        else:
            hours_passed = (now - user.userdata.urgency_calculated_at).total_seconds() / 3600
            if hours_passed >= DAY_HOURS:
                needs_calculation = True

        if not needs_calculation:
            continue

        calculate_score_for_user(user_data, now)
