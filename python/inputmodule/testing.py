import math
import serial
from serial.tools import list_ports

import gui  # Needed for sketchy reasons.
from inputmodule import INPUTMODULE_PIDS
from inputmodule.ledmatrix import send_col, commit_cols, WIDTH, HEIGHT

def find_devs():
    ports = list_ports.comports()
    return [
        port for port in ports if port.vid == 0x32AC and port.pid in INPUTMODULE_PIDS
    ]

def perceptual_brightness(brightness):
    # k = 1/255 * (math.log(3) + math.log(5) + math.log(17))
    # return round(math.exp(k * brightness))
    return round((1/255) * brightness * brightness)
    # return round(math.pow(brightness - 128, 3) / 32800 + 0.5 * brightness + 64)
    # return round(math.pow(brightness - 192, 3) / 120000 + 0.5 * brightness + 64)
    # return brightness

if __name__ == '__main__':
    for port in find_devs():
        print(port.device)
    
    device = find_devs()[0].device
    with serial.Serial(device, 115200) as s:
        for col in range(WIDTH):
            value = round(255 - (255 / WIDTH * col))
            print(value)
            send_col(device, s, col, [perceptual_brightness(value - (row * 5)) for row in range(HEIGHT)])
        commit_cols(device, s)
