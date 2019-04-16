# -*- coding: utf-8 -*-
from time import sleep
from serial import Serial


def main():
    # type: () -> None
    with Serial("/dev/ttyACM0", 115200) as com:
        while True:
            print("".join(read_message(com)))
            sleep(0.1)


def read_message(com):
    while True:
        if com.in_waiting:
            buf = com.read(1)
            if buf == b"\r":
                break
            yield chr(buf[0])


def read_bytes(com):
    is_stx = False
    while True:
        sleep(0.1)
        if com.in_waiting == 0:
            continue
        x = com.read(1)
        if is_started is False:
            if x == b"\x02":
                is_stx = True
                continue
        yield com.read(1)
        yield com.read(1)
        yield com.read(1)
        if x == b"\x03":
            break
        yield chr(x)


if __name__ == '__main__':
    main()
