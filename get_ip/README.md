# Get_IP

## Overview
A CLI that returns system's local IP address and network interfaces. Users enter a command from the command line, and the program returns the IP address and network interfaces.



## Usage
1. cd into the get_ip directory `get_ip`

    ```bash
    cd get_ip
    ```

2. To run the program, type `cargo run -- get_ip`. The program will return the IP address and network interfaces.

    For example, to look up the local IP address, type:

    ```bash
    cargo run -- localip
    ```

    And the program will return:

    ```bash
    IP Address:192.168.1.90
    ```

    For example, to look up the network interfaces, type:

    ```bash
    cargo run -- listnetifas
    ```
    And the program will return:

    ```bash
    utun3:  fe80::9ee6:a113:8bc6:7b69
    utun4:  fe80::970e:20d4:4bb6:f8c9
    utun5:  fe80::67c3:389f:3f9a:a943
    utun6:  fe80::5873:ba58:5d42:38f4
    utun7:  fe80::5ee6:2373:a64e:4739
    utun8:  fe80::e2a5:4383:3cbc:1aa4
    ```

  