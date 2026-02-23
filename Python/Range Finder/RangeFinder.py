import requests

for i in range(0, 271000, 12):
    r = requests.get("http://10.35.23.250/", headers={"Range" : f"{i}-{i+12}"})
    if "CS" in r.content.decode():
        print(f"\n\n{i}-{i+12}\n\n")
        print(r.content.decode())