# Generated by Django 2.2.4 on 2019-08-31 22:25

from django.db import migrations, models


# fmt: off
class Migration(migrations.Migration):

    dependencies = [
        ('hawk_gui', '0001_initial'),
    ]

    operations = [
        migrations.AddField(
            model_name='report',
            name='uuid',
            field=models.UUIDField(default='00000000000000000000000000000000', verbose_name='UUID'),
            preserve_default=False,
        ),
    ]