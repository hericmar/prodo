# Generated by Django 4.2.2 on 2023-07-17 17:48

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('tasks', '0006_alter_tasklist_ordered_tasks'),
    ]

    operations = [
        migrations.AlterField(
            model_name='tasklist',
            name='ordered_tasks',
            field=models.JSONField(blank=True, default=list),
        ),
    ]
