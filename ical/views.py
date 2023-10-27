import datetime
from io import StringIO

from django.http import HttpResponse
from rest_framework import status
from rest_framework.decorators import api_view, permission_classes, throttle_classes
from rest_framework.generics import DestroyAPIView, RetrieveAPIView, CreateAPIView, get_object_or_404
from rest_framework.permissions import IsAuthenticated
from rest_framework.response import Response
from rest_framework.throttling import UserRateThrottle

from base.serializers import EmptySerializer
from ical.models import Subscription
from ical.serializers import SubscriptionSerializer, PostSubscriptionSerializer
from tasks.models import Task


DUE_DURATION_HOURS = 4


class SubscriptionView(RetrieveAPIView, CreateAPIView, DestroyAPIView):
    serializer_class = SubscriptionSerializer
    permission_classes = [IsAuthenticated]

    def get_serializer_class(self):
        if self.request.method == 'POST':
            return PostSubscriptionSerializer
        return SubscriptionSerializer

    def get_queryset(self):
        return Subscription.objects.filter(user=self.request.user)

    def get_object(self):
        """
        Used by RetrieveAPIView to retrieve the object for this view.
        """
        return get_object_or_404(self.get_queryset())

    def perform_create(self, serializer):
        if Subscription.objects.filter(user=self.request.user).exists():
            return Response(
                {"error": "subscription already exists"},
                status=status.HTTP_409_CONFLICT
            )

        Subscription.objects.create(
            created_by=self.request.user,
            user=self.request.user,
            secret=Subscription.generate_secret()
        )

    def perform_destroy(self, instance):
        if not Subscription.objects.filter(user=self.request.user).exists():
            return Response(
                {"error": "subscription does not exists"},
                status=status.HTTP_404_NOT_FOUND
            )

        Subscription.objects.filter(user=self.request.user).delete()


@api_view(['GET'])
@throttle_classes([UserRateThrottle])
def ical_view(request, secret: str):
    subscription = get_object_or_404(Subscription, secret=secret)
    if subscription.secret != secret:
        return Response(
            {"error": ""},
            status=status.HTTP_404_NOT_FOUND
        )

    tasks = Task.objects.get_active(user=subscription.user)

    # Template with required fields
    EVENT_TEMPLATE = """BEGIN:VEVENT
UID:{uid}
DTSTAMP:{created}
SEQUENCE:{sequence}
SUMMARY:{summary}
DESCRIPTION:{description}"""

    events = []

    # dtstart is required by ical spec
    tasks = filter(lambda task: task.start is not None or task.due is not None, tasks)

    for task in tasks:
        event_data = {**task.__dict__}

        event_data['created'] = event_data['created'].strftime('%Y%m%dT%H%M%SZ')

        lines = [EVENT_TEMPLATE.format(**event_data)]

        # `dtend` is optional, `dtstart` can be used with duration or rrule
        if event_data.get('start') and event_data.get('end'):
            lines.append("\nDTSTART:" + event_data['start'].strftime('%Y%m%dT%H%M%SZ'))
            lines.append("\nDTEND:" + event_data['end'].strftime('%Y%m%dT%H%M%SZ'))
        elif event_data.get('due'):
            end = event_data['due']
            start = end - datetime.timedelta(hours=DUE_DURATION_HOURS)
            lines.append("\nDTSTART:" + start.strftime('%Y%m%dT%H%M%SZ'))
            lines.append("\nDTEND:" + end.strftime('%Y%m%dT%H%M%SZ'))

        if event_data['rrule']:
            lines.append('\n' + event_data['rrule'])

        lines.append("\nEND:VEVENT")

        events.append(''.join(lines))

    data = """BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//Phire//Prodo//EN
{events}
END:VCALENDAR
""".format(events='\n'.join(events))

    return HttpResponse(data.replace('\n', '\r\n'), content_type='text/calendar')
