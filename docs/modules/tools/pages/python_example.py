import json

#with open("/var/log/umrs-activity.jsonl", "r", encoding="utf-8") as f:
with open("./test.jsonl", "r", encoding="utf-8") as f:
    event = {}
    for line in f:
        line = line.strip()
        if not line:
            continue
        event = json.loads(line)

    print(event)
