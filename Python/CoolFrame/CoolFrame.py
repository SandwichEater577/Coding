import platform
import time
import subprocess

def frameAnimation(message, frames=4, speed=0.05):
    for frame in ["|", "/", "-", "\\"] * frames:
        subprocess.call('cls' if platform.system() == "Windows" else 'clear', shell=True)
        print(f"{message}: {frame}")
        time.sleep(speed)
    subprocess.call('cls' if platform.system() == "Windows" else 'clear', shell=True)   