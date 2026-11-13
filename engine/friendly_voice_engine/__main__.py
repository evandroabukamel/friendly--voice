"""Command line: ``friendly-voice-engine download`` and ``friendly-voice-engine serve``."""

from __future__ import annotations

import argparse

from .config import Settings
from .download import download_all


def main() -> None:
    parser = argparse.ArgumentParser(prog="friendly-voice-engine", description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("download", help="download the model weights into the Hugging Face cache")
    serve = sub.add_parser("serve", help="start the HTTP server")
    serve.add_argument("--host", default="127.0.0.1")
    serve.add_argument("--port", type=int, default=8020)
    args = parser.parse_args()

    if args.command == "download":
        download_all()
        print("models ready")
    else:
        import uvicorn

        from .server import create_app

        uvicorn.run(create_app(settings=Settings.from_env()), host=args.host, port=args.port)


if __name__ == "__main__":
    main()
