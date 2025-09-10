import math

from pyorion import command




@command
async def compute_function(func: str) -> dict:
    """Berechnet eine wissenschaftliche Funktion.
    Unterstützt: sin, cos, exp, fourier.
    """
    xs = [i * 0.1 for i in range(-100, 101)]

    if func == "sin":
        ys = [math.sin(x) for x in xs]
    elif func == "cos":
        ys = [math.cos(x) for x in xs]
    elif func == "exp":
        ys = [math.exp(x / 10) for x in xs]
    elif func == "fourier":
        ys = []
        for x in xs:
            s = 0
            for n in range(1, 10, 2):  # Fourier-Reihe einer Rechteckfunktion
                s += (4 / (math.pi * n)) * math.sin(n * x)
            ys.append(s)
    else:
        raise ValueError("Unbekannte Funktion")

    return {"x": xs, "values": ys}
