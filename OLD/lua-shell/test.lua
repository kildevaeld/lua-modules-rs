local shell = require 'core.shell'


local status = shell.exec("ls ./"):pipe(shell.exec("rofi -dmenu")):status()


print("status " .. status)
