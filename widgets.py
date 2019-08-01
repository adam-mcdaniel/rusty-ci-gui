from tkinter import Tk, Label, Button, Frame, Widget, Entry, Text, Listbox, OptionMenu, StringVar, END


class GuiWidget(Frame, Widget):
    def __init__(self, master, *args, **kwargs):
        super().__init__(master)
        self.setup(master, *args, **kwargs)
        self.pack()

    def setup(self, *args, **kwargs): pass


class ListBox(GuiWidget):
    def setup(self, master, items, *args, **kwargs):
        self.listbox = Listbox(master)
        self.listbox.pack()

        for item in items:
            self.listbox.insert(END, item)


class DropDown(GuiWidget):
    def setup(self, master, items, *args, **kwargs):
        self.tkvar = StringVar(master)

        self.tkvar.set(items[0])

        self.popupMenu = OptionMenu(master, self.tkvar, *items)
        self.popupMenu.pack()

    def __str__(self): return self.tkvar.get()


class TextEntry(GuiWidget):
    def setup(self, master, *args, **kwargs):
        self.content = StringVar()
        self.entry = Entry(master, textvariable=self.content)
        self.entry.pack()

    def __str__(self): return self.content.get()


class MultilineEntry(GuiWidget):
    def setup(self, master, *args, **kwargs):
        self.entry = Text(master)
        self.entry.pack()

    def __str__(self): return self.entry.get(1.0, END)[:-1]
