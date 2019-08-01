# The goal of this particular file is to provide a means
# to move to another gui backend. If we do need to switch, 
# we should be able to reimplement the primitives and 
# everything else can remain unchanged.
from tkinter import Tk, Label, Button, Frame, Widget, Entry, Text, Listbox, OptionMenu, StringVar, END


# The base class for all Widgets to be displayed
# The purpose of this class is to provide a means to compose widgets
# The goal is to create widgets by only implementing different
# combinations of primitive widgets, and other custom widgets also
# built on top of primitive widgets.
class GuiWidget(Frame, Widget):
    def __init__(self, master, *args, **kwargs):
        super().__init__(master)
        self.setup(master, *args, **kwargs)
        self.pack()

    # This is to act as the __init__ method for the GuiWidget subclasses
    def setup(self, *args, **kwargs): pass


# A list of items to choose from as a dropdown
class DropDown(GuiWidget):
    def setup(self, master, items, *args, **kwargs):
        # Stores current selection
        self.selection = StringVar(master)
        self.selection.set(items[0])

        # The menu widget
        self.popupMenu = OptionMenu(master, self.selection, *items)
        self.popupMenu.pack()

    def __str__(self): return str(self.selection.get())


# Enter one line into a text dialog box
class LineEntry(GuiWidget):
    def setup(self, master, *args, **kwargs):
        self.content = StringVar()
        self.entry = Entry(master, textvariable=self.content)
        self.entry.pack()

    def __str__(self): return self.content.get()


# Enter multiple lines into a dialog box
class MultilineEntry(GuiWidget):
    def setup(self, master, *args, **kwargs):
        self.entry = Text(master)
        self.entry.pack()

    def __str__(self): return self.entry.get(1.0, END)[:-1]
