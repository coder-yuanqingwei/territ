"""Synthesize game sound effects into assets/sfx/*.wav (16-bit mono, 22050 Hz).

Run: python3 tools/gen_sfx.py
"""

import math
import random
import struct
import wave
from pathlib import Path

SR = 22050
OUT = Path(__file__).resolve().parent.parent / "assets" / "sfx"


def write_wav(name: str, samples: list[float], gain: float = 0.8) -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    path = OUT / name
    with wave.open(str(path), "w") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(SR)
        frames = b"".join(
            struct.pack("<h", max(-32767, min(32767, int(s * gain * 32767))))
            for s in samples
        )
        w.writeframes(frames)
    print(f"wrote {path} ({path.stat().st_size} bytes)")


def tone(segments, shape="sine", decay=8.0) -> list[float]:
    samples = []
    for freq, dur in segments:
        n = int(SR * dur)
        for i in range(n):
            t = i / SR
            env = math.exp(-decay * t)
            ph = 2.0 * math.pi * freq * t
            if shape == "sine":
                v = math.sin(ph)
            elif shape == "square":
                v = 1.0 if math.sin(ph) >= 0.0 else -1.0
            else:
                v = 2.0 / math.pi * math.asin(math.sin(ph))
            samples.append(v * env)
    return samples


def sweep(f0: float, f1: float, dur: float, decay=4.0) -> list[float]:
    samples = []
    phase = 0.0
    n = int(SR * dur)
    for i in range(n):
        t = i / n
        f = f0 * (f1 / f0) ** t
        phase += 2.0 * math.pi * f / SR
        samples.append(math.sin(phase) * math.exp(-decay * (i / SR)))
    return samples


def gen_rotate() -> list[float]:
    return tone([(660.0, 0.05), (880.0, 0.06)], shape="square", decay=18.0)


def gen_hold() -> list[float]:
    return tone([(440.0, 0.06), (330.0, 0.09)], shape="tri", decay=12.0)


def gen_hard_drop() -> list[float]:
    random.seed(42)
    n = int(SR * 0.12)
    samples = []
    for i in range(n):
        t = i / SR
        thump = math.sin(2.0 * math.pi * 110.0 * t) * math.exp(-20.0 * t)
        noise = random.uniform(-1.0, 1.0) * math.exp(-40.0 * t) * 0.5
        samples.append(thump + noise)
    return samples


def gen_clear() -> list[float]:
    return tone(
        [(523.25, 0.07), (659.25, 0.07), (783.99, 0.12)],
        shape="tri",
        decay=6.0,
    )


def gen_level_up() -> list[float]:
    return tone(
        [(523.25, 0.08), (659.25, 0.08), (783.99, 0.08), (1046.5, 0.16)],
        shape="tri",
        decay=5.0,
    )


def gen_game_over() -> list[float]:
    return sweep(400.0, 80.0, 0.55, decay=3.0)


def main() -> None:
    write_wav("rotate.wav", gen_rotate())
    write_wav("hold.wav", gen_hold())
    write_wav("harddrop.wav", gen_hard_drop())
    write_wav("clear.wav", gen_clear())
    write_wav("levelup.wav", gen_level_up())
    write_wav("gameover.wav", gen_game_over())


if __name__ == "__main__":
    main()
