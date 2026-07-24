import queue
import sys

import numpy as np
import sounddevice as sd


class MicrophoneStream:
    def __init__(
        self,
        sample_rate: int = 16000,
        chunk_size: int = 1280,
        device: int | None = None,
    ):
        self.sample_rate = sample_rate
        self.chunk_size = chunk_size
        self.device = device
        self.audio_queue: queue.Queue[np.ndarray] = queue.Queue()
        self.stream: sd.InputStream | None = None

    def _callback(self, indata, frames, callback_time, status) -> None:
        if status:
            print(status, file=sys.stderr)

        mono_chunk = indata[:, 0].copy()
        self.audio_queue.put(mono_chunk)

    def start(self) -> None:
        self.stream = sd.InputStream(
            samplerate=self.sample_rate,
            channels=1,
            dtype="int16",
            blocksize=self.chunk_size,
            device=self.device,
            callback=self._callback,
        )
        self.stream.start()

    def stop(self) -> None:
        if self.stream is None:
            return

        self.stream.stop()
        self.stream.close()
        self.stream = None

    def read_chunk(self) -> np.ndarray:
        return self.audio_queue.get()
