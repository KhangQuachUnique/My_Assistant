import argparse
import json
import sys
import time

import sounddevice as sd
from recorder import MicrophoneStream
from wake_word import WakeWordDetector

SAMPLE_RATE = 16000
CHUNK_SIZE = 1280


def emit(event_type: str, **payload) -> None:
    print(
        json.dumps(
            {
                "type": event_type,
                **payload,
            },
            ensure_ascii=False,
        ),
        flush=True,
    )


def list_devices() -> None:
    print(sd.query_devices())


def main() -> None:
    parser = argparse.ArgumentParser(description="Voice worker for desktop assistant.")
    parser.add_argument(
        "--model",
        default="data/models/wake_word/hey_teo_test.onnx",
        help="Path to the wake word ONNX model.",
    )
    parser.add_argument(
        "--threshold",
        type=float,
        default=0.1,
        help="Wake word detection threshold.",
    )
    parser.add_argument(
        "--device",
        type=int,
        default=None,
        help="Input device index.",
    )
    parser.add_argument(
        "--list-devices",
        action="store_true",
        help="Print audio devices and exit.",
    )

    args = parser.parse_args()

    if args.list_devices:
        list_devices()
        return

    wake = WakeWordDetector(
        model_path=args.model,
        threshold=args.threshold,
    )

    mic = MicrophoneStream(
        sample_rate=SAMPLE_RATE,
        chunk_size=CHUNK_SIZE,
        device=args.device,
    )

    emit("status", value="starting")
    emit("wake_model_loaded", model=wake.model_name, threshold=args.threshold)

    mic.start()
    emit("status", value="listening")

    last_score_print = 0.0

    try:
        while True:
            chunk = mic.read_chunk()
            detected, score = wake.predict(chunk)
            now = time.time()

            if detected:
                emit("wake_word_detected", model=wake.model_name, score=score)
                continue

            if now - last_score_print >= 0.5:
                emit("wake_score", score=score)
                last_score_print = now

    except KeyboardInterrupt:
        emit("status", value="stopped")
    except Exception as error:
        emit("error", message=str(error))
        print(str(error), file=sys.stderr)
    finally:
        mic.stop()


if __name__ == "__main__":
    main()
