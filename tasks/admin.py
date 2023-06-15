from django.contrib import admin
from django.forms import forms

from tasks.models import Task, TaskEvent

# See https://books.agiliq.com/projects/django-admin-cookbook/en/latest/index.html


# update task form to autofill created_by field by with current user
class TaskAdmin(admin.ModelAdmin):
    readonly_fields = ('created_by', 'updated_by', 'sequence')

    def save_model(self, request, obj, form, change):
        if not change:
            # create
            obj.created_by = request.user
        else:
            # update
            obj.updated_by = request.user
        obj.save()


admin.site.register(Task, TaskAdmin)
admin.site.register(TaskEvent)

"""
class ProductCreateForm(forms.ModelForm):
    class Meta:
        model = Product
        fields = ('name', 'description', 'price')

class ProductChangeForm(forms.ModelForm):
    class Meta:
        model = Product
        fields = ('name', 'description')

class ProductAdmin(admin.ModelAdmin):
    def get_form(self, request, obj=None, **kwargs):
        if obj is None:
            # Use custom form for create
            return ProductCreateForm
        else:
            # Use custom form for change
            return ProductChangeForm
"""
