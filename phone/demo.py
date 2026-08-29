import subprocess
import sys

import needle


@needle.tool
def text_to_speech(text: str):
    """Use text-to-speech to speak a message aloud."""
    subprocess.run(["termux-tts-speak", text])
    return f"Spoken: {text}"


@needle.tool
def battery_status():
    """Get the current battery status of the phone."""
    result = subprocess.run(["termux-battery-status"], capture_output=True, text=True)
    return result.stdout


@needle.tool
def vibrate(duration: int):
    """Vibrate the phone for a specified duration in milliseconds."""
    subprocess.run(["termux-vibrate", "-d", str(duration)])
    return f"Vibrated for {duration}ms"


@needle.tool
def set_volume(level: int):
    """Set the phone volume to a specific level (0-100)."""
    subprocess.run(["termux-volume", "music", str(level)])
    return f"Volume set to {level}%"


agent = needle.Needle(tools=[text_to_speech, battery_status, vibrate, set_volume])

prompt = sys.argv[1] if len(sys.argv) > 1 else input("Ask: ")
response = agent.run(prompt)
print(response.result)
