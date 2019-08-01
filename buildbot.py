from primitives import *


class ScriptInput(GuiWidget):
    def setup(self, master, *args, **kwargs):
        self.script = MultilineEntry(master)
        self.script.pack()

    def __str__(self): return str(self.script)


class VCS(GuiWidget):
    def setup(self, master):
        self.dropdown = DropDown(master, ["github", "gitlab"])
    
    def __str__(self): str(self.dropdown)