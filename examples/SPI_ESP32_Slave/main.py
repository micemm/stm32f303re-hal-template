# main.py
# main file:

# init prg pin
from machine import Pin
import sys
import machine
import time

# setup interrupt for repl at button click
prg = Pin(0, Pin.IN)
running = True

def exit_to_repl(p):
    print("Exiting to repl...")
    global running
    running = False

prg.irq(trigger=Pin.IRQ_FALLING, handler=exit_to_repl)

# main code...
import wifi_credentials
import network
import ntptime

def main():
    global running

    # setup network connection
    nic = network.WLAN(network.STA_IF)
    nic.active(True)
    nic.connect(wifi_credentials.SSID, wifi_credentials.KEY)
    print("Connecting to wifi ", end="")
    while not nic.isconnected():
        print(".", end="")
        time.sleep(1)
    print()
    print("Connected to WiFi")
    
    while running:
        print(ntptime.time())
        time.sleep(1)

main()