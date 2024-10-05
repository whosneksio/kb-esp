# kb-esp

## closet esp for csgo, as a neverlose lua script, for alloy origins fps 60

### how it works:

- it opens a web server on local computer as neverlose api doesnt allow opening external programs
- it scans distance between you and a local player, and sends it to the webserver, the web sever depening on value changes the color of your keyboard depeding on the distance between you and closes enemy

### how to use:

- compile the hid using `cargo build` and move the hid.exe to the directory with server.py
- create a new script in neverlose and put the nl/script.lua content into it
- launch the web server by running `server.py`
- launch the script in neverlose

### will it be updated

- no, it was purely made for educational and fun purposes

#### credits:

https://github.com/EriksRemess/hyperrust \
https://lua.neverlose.cc/useful-information/script-examples/vector

* yes i know this project is pointless