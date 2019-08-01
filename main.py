from tkinter import Tk, Label, Button, Frame, Widget, Entry, Text, StringVar, END
from tkinter.ttk import *

from buildbot import ScriptInput, VCS
from widgets import ListBox, DropDown


class RustyCIGui:
    def __init__(self, master):
        self.master = master
        master.title("RustyCI GUI")

        self.script = ScriptInput(master)
        self.listbox = VCS(master)


root = Tk()
root.geometry("320x480")

RustyCIGui(root)
root.style = Style()
root.style.theme_use("default")
root.mainloop()