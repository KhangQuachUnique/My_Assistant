import time

import numpy as np
from openwakeword.model import Model


class WakeWordDetector:
    def __init__(
        self,
        model_path: str,
        threshold: float = 0.1,
        cooldown_seconds: float = 1.0,
    ):
        self.model = Model(
            wakeword_models=[model_path],
            inference_framework="onnx",
        )
        self.model_name = next(iter(self.model.models.keys()))
        self.threshold = threshold
        self.cooldown_seconds = cooldown_seconds
        self.last_detection = 0.0

    def predict(self, audio_chunk: np.ndarray) -> tuple[bool, float]:
        predictions = self.model.predict(audio_chunk)

        score = float(predictions.get(self.model_name, 0.0))
        now = time.time()

        detected = (
            score >= self.threshold
            and now - self.last_detection > self.cooldown_seconds
        )

        if detected:
            self.last_detection = now

        return detected, score
