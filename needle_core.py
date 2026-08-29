import needle
from tools import ALL_TOOLS


_agent = None


def get_agent():
    """Get or create the Needle agent with all tools."""
    global _agent
    if _agent is None:
        _agent = needle.Needle(
            tools=ALL_TOOLS,
            tool_index_path="tools.idx"
        )
    return _agent


def ask(query: str) -> dict:
    """Ask a question and return the response."""
    agent = get_agent()
    return agent.run(query)
