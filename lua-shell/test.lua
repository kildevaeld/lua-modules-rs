local shell = require 'shell'


shell.ls("./"):pipe(shell.exec("rofi -dmenu"))
