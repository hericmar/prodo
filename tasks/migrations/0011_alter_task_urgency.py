# Generated by Django 4.2.2 on 2023-07-25 22:07

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('tasks', '0010_task_urgency_alter_task_priority_alter_task_score'),
    ]

    operations = [
        migrations.AlterField(
            model_name='task',
            name='urgency',
            field=models.PositiveSmallIntegerField(default=0),
        ),
    ]
