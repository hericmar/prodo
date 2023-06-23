from django.http import HttpResponse
from rest_framework.generics import CreateAPIView, ListAPIView, RetrieveAPIView, UpdateAPIView, DestroyAPIView
from rest_framework.permissions import IsAuthenticated

from tasks.models import Task
from tasks.serializers import TaskSerializer


class TaskListView(ListAPIView, CreateAPIView):
    serializer_class = TaskSerializer
    permission_classes = [IsAuthenticated]

    def get_queryset(self):
        """
        This view should return a list of all the active tasks
        for the currently authenticated user.
        """
        return Task.objects.filter(created_by=self.request.user, active=True)

    def perform_create(self, serializer):
        serializer.save(created_by=self.request.user)


class TaskDetailView(RetrieveAPIView, DestroyAPIView, UpdateAPIView):
    serializer_class = TaskSerializer
    permission_classes = [IsAuthenticated]
    lookup_field = 'uid'

    def get_queryset(self):
        return Task.objects.filter(created_by=self.request.user)

    def perform_update(self, serializer):
        serializer.save(updated_by=self.request.user)

    def perform_destroy(self, instance):
        instance.active = False
        instance.save()


def ical_view(request):
    # tasks = Task.objects.filter(created_by=request.user.id)
    tasks = Task.objects.filter(active=True)

    EVENT_TEMPLATE = """BEGIN:VEVENT
DTSTAMP:{created}
SUMMARY:{summary}
DESCRIPTION:{description}
UID:{uid}"""

    events = []
    for task in tasks:
        event_data = {**task.__dict__}

        if not event_data['start']:
            continue

        # event_data['uid'] = str(event_data['uid']) + '@prodo'
        event_data['created'] = event_data['created'].strftime('%Y%m%dT%H%M%SZ')

        event = EVENT_TEMPLATE.format(**event_data)

        if 'start' in event_data and 'end' in event_data:
            event = event + "\nDTSTART:" + event_data['start'].strftime('%Y%m%dT%H%M%SZ')
            event = event + "\nDTEND:" + event_data['end'].strftime('%Y%m%dT%H%M%SZ')

        if event_data['rrule']:
            event += "RRULE:" + event_data['rrule']

        event += "\nEND:VEVENT"

        events.append(event)

    data = """BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//Prodo//Tasks//EN
{events}
END:VCALENDAR
""".format(events='\nEND:VEVENT\n'.join(events))

    return HttpResponse(data.replace('\n', '\r\n'), content_type='text/calendar')
