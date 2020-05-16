from datetime import datetime
from typing import List

from dateutil.relativedelta import relativedelta
from flask import Flask
app = Flask('christmas-countdown')

@app.route('/')
def christmas_countdown_handler():
    now = datetime.now()
    christmas = get_next_christmas(now)
    if now.month == christmas.month and now.day == christmas.day:
        return "IT'S CHRISTMAS!"

    time_until_christmas = human_readable_timedelta(christmas, now)
    return f'Only {time_until_christmas} until christmas! :christmas_tree:'

def get_next_christmas(d: datetime) -> datetime:
    if d.month >= 12 and d.day >= 26:
        return datetime(d.year+1, 12, 25)
    else:
        return datetime(d.year, 12, 25)

def human_readable_timedelta(christmas: datetime, d: datetime) -> str:
    difference = relativedelta(christmas, d)
    components = []

    months = "months" if difference.months != 1 else "month"
    components.append(f"{difference.months} {months}")

    days = "days" if difference.days != 1 else "day"
    components.append(f"{difference.days} {days}")

    hours = "hours" if difference.hours != 1 else "hour"
    components.append(f"{difference.hours} {hours}")

    minutes = "minutes" if difference.minutes != 1 else "minute"
    components.append(f"{difference.minutes} {minutes}")

    seconds = "seconds" if difference.seconds != 1 else "second"
    components.append(f"{difference.seconds} {seconds}")

    microseconds = "microseconds" if difference.microseconds != 1 else "microsecond"
    components.append(f"{difference.microseconds} {microseconds}")

    # gotta remove any leading "0 months" e.g.
    def truncate(l: List[str]) -> list:
        truncated = False
        for s in l:
            if truncated:
                yield s
            elif len(s) > 0 and s[0] != "0":
                truncated = True
                yield s

    truncated_components = list(truncate(components))

    if len(truncated_components) == 1:
        return truncated_components[0]
    elif len(truncated_components) == 2:
        return " and ".join(truncated_components)
    elif len(truncated_components) > 2:
        truncated_components[-1] = "and " + truncated_components[-1]
        return ", ".join(truncated_components)
