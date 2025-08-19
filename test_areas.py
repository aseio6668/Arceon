#!/usr/bin/env python3
"""
Quick test script to demonstrate the Arceon area system
"""

import subprocess
import time

def run_test():
    print("Testing Arceon Terminal with different races...")
    
    # Test Human starting area
    print("\n=== Testing Human (Race 1) ===")
    proc = subprocess.Popen(
        ["cargo", "run", "--bin", "arceon-terminal"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        cwd=r"c:\Users\aseio\Documents\.Source\Arceon[rust]"
    )
    
    # Send race selection and commands
    commands = "1\nlook\nwho\napproach harbor\nquit\n"
    stdout, stderr = proc.communicate(input=commands, timeout=30)
    
    print("Human output:")
    print(stdout.split("Type 'help' for available commands")[1].split("Farewell")[0] if "Type 'help'" in stdout else "Error running")
    
    # Test Elf starting area
    print("\n=== Testing Elf (Race 2) ===")
    proc = subprocess.Popen(
        ["cargo", "run", "--bin", "arceon-terminal"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        cwd=r"c:\Users\aseio\Documents\.Source\Arceon[rust]"
    )
    
    commands = "2\nlook\nwho\nquit\n"
    stdout, stderr = proc.communicate(input=commands, timeout=30)
    
    print("Elf output:")
    print(stdout.split("Type 'help' for available commands")[1].split("Farewell")[0] if "Type 'help'" in stdout else "Error running")

if __name__ == "__main__":
    try:
        run_test()
    except Exception as e:
        print(f"Test failed: {e}")
