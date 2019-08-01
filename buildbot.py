from primitives import *


class ScriptInput(GuiWidget):
    def setup(self, master, *args, **kwargs):
        self.script = MultilineEntry(master)
        self.script.pack()

    def __str__(self): return str(self.script)


class VCS(DropDown):
    def setup(self, master):
        super().setup(master, ["github", "gitlab"])