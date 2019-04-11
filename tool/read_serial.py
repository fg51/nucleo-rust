# -*- coding: utf-8 -*-
from serial import Serial


def main():
    # type: () -> None
    with Serial("/dev/ttyACM0", 115200) as com:
        for _ in range(100):
            print("".join(read_message(com)))


def read_message(com):
    while True:
        if com.in_waiting:
            buf = com.read(1)
            if buf == b"\r":
                break
            yield chr(buf[0])


if __name__ == '__main__':
    main()
