"""
URL configuration for prodo project.

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/4.2/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""
from django.contrib import admin
from django.http import HttpResponse
from django.template import engines
from django.conf.urls.static import static
from django.urls import path, include
from rest_framework import routers

from prodo import settings
from tasks.views import TaskListView, TaskDetailView

router = routers.DefaultRouter()


def index_view(request):
    with open('static/index.html') as file:
        template_string = file.read()

    django_engine = engines['django']
    template = django_engine.from_string(template_string)

    # Render the template with some context data
    context = {'base_url': request.get_host()}
    rendered_template = template.render(context)

    BASE_URL = """
<script>
  Object.defineProperty(window, '""" + request.get_host() + """', {
    value: 'nginx.conf',
    writable: false
  });
</script>
    """

    rendered_template = rendered_template.replace('<!-- prodo:base-url -->', BASE_URL)

    # Further processing or return the rendered template
    return HttpResponse(rendered_template)


urlpatterns = [
    path('', index_view),
    path('admin/', admin.site.urls),
    path('api/v1', include(router.urls)),
    path('api/v1/auth/', include('users.urls')),
    path('api/v1/tasks', TaskListView.as_view()),
    path('api/v1/tasks/<uid>', TaskDetailView.as_view()),
] + static(settings.STATIC_URL, document_root=settings.STATIC_ROOT)
