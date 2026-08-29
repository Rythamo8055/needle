import json
import subprocess
import sys


def run_needle(prompt: str, tools_file: str = "tools.json"):
    """Run the needle binary with a prompt and tools definition."""
    result = subprocess.run(
        ["./needle", "--tools", tools_file, "--prompt", prompt],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        print(f"Error: {result.stderr}", file=sys.stderr)
        return None

    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError:
        return result.stdout


if __name__ == "__main__":
    prompt = sys.argv[1] if len(sys.argv) > 1 else input("Ask: ")
    result = run_needle(prompt)
    if result:
        print(result)
