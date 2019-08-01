from tkinter import Tk, Label, Button, Frame, Widget, Entry, Text, StringVar, END
from tkinter.ttk import *


from buildbot import ScriptInput, VCS


class RustyCIGui:
    def __init__(self, master):
        self.master = master
        master.title("RustyCI GUI")

        self.script = ScriptInput(master)
        self.vcs = VCS(master)

        self.button = Button(master, text="Print info", command=self.print_info)
        self.button.pack()

    def print_info(self):
        # print(f"Script: '{str(self.script)}'\n\nVCS: '{str(self.vcs)}'")
        print(f"Script: '{str(self.script)}'\nVCS: '{str(self.vcs)}'\n\n\n")
        


root = Tk()
root.geometry("320x480")

RustyCIGui(root)
root.style = Style()
root.style.theme_use("default")
root.mainloop()