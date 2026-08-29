import gi
gi.require_version("Gtk", "4.0")
from gi.repository import Gtk, GLib
import threading
import needle_core
import tools


class NeedleApp(Gtk.Application):
    def __init__(self):
        super().__init__(application_id="com.needle.sysinfo")
        self.connect("activate", self.on_activate)

    def on_activate(self, app):
        win = Gtk.ApplicationWindow(application=app, title="Needle — System Info")
        win.set_default_size(700, 500)

        vbox = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=8)
        vbox.set_margin_top(12)
        vbox.set_margin_bottom(12)
        vbox.set_margin_start(12)
        vbox.set_margin_end(12)
        win.set_child(vbox)

        # Search bar
        self.entry = Gtk.SearchEntry()
        self.entry.set_placeholder_text("Ask about your system... (e.g. 'battery health', 'trash size')")
        self.entry.connect("activate", self.on_search)
        vbox.append(self.entry)

        # Quick buttons
        flow = Gtk.FlowBox()
        flow.set_selection_mode(Gtk.SelectionMode.NONE)
        flow.set_column_spacing(6)
        flow.set_row_spacing(6)
        for label in ["Battery status", "Trash size", "CPU temperature",
                       "Memory usage", "Disk usage", "Top processes",
                       "Network info", "GPU info", "Uptime", "Disk health"]:
            btn = Gtk.Button(label=label)
            btn.connect("clicked", self.on_quick_button, label.lower())
            flow.append(btn)
        vbox.append(flow)

        # Scrolled results
        scroll = Gtk.ScrolledWindow()
        scroll.set_vexpand(True)
        self.result_label = Gtk.Label()
        self.result_label.set_wrap(True)
        self.result_label.set_xalign(0)
        self.result_label.set_selectable(True)
        self.result_label.set_markup('<span size="large" style="italic">Ask something to get started...</span>')
        scroll.set_child(self.result_label)
        vbox.append(scroll)

        # Status bar
        self.status = Gtk.Label()
        self.status.set_xalign(0)
        self.status.set_markup('<span size="small" foreground="#888">Ready</span>')
        vbox.append(self.status)

        win.present()

        # Check sudo in background
        threading.Thread(target=self._check_sudo, daemon=True).start()

    def on_quick_button(self, button, query):
        self.entry.set_text(query)
        self.on_search(self.entry)

    def _check_sudo(self):
        has_sudo = tools.check_sudo()
        if not has_sudo:
            GLib.idle_add(self._sudo_prompt)

    def _sudo_prompt(self):
        dialog = Gtk.AlertDialog()
        dialog.set_message("Sudo Access Needed")
        dialog.set_detail("Some tools need sudo. Run 'sudo true' in a terminal to cache your password, then restart the app.")
        dialog.set_buttons(["OK"])
        dialog.show(self.get_active_window())
        return False

    def on_search(self, entry):
        query = entry.get_text().strip()
        if not query:
            return
        self.status.set_markup(f'<span size="small" foreground="#888">Thinking...</span>')
        self.result_label.set_markup('<span size="large" style="italic">Loading model...</span>')
        threading.Thread(target=self._run_query, args=(query,), daemon=True).start()

    def _run_query(self, query):
        try:
            response = needle_core.ask(query)
            results = response.get("results", [])
            reasoning = response.get("reasoning", "")
            confidence = response.get("confidence", 0)
            if isinstance(results, list):
                text = "\n\n".join(str(r) for r in results)
            else:
                text = str(results)
            display = f"<b>Query:</b> {GLib.markup_escape_text(query)}\n\n"
            display += f"<b>Result:</b>\n{GLib.markup_escape_text(text)}\n\n"
            display += f'<span foreground="#888" size="small">Confidence: {confidence:.0%} | {GLib.markup_escape_text(reasoning)}</span>'
        except Exception as e:
            display = f'<span foreground="red">Error: {GLib.markup_escape_text(str(e))}</span>'
        GLib.idle_add(self._update_ui, display)

    def _update_ui(self, markup):
        self.result_label.set_markup(markup)
        self.status.set_markup('<span size="small" foreground="#888">Done</span>')
        return False


if __name__ == "__main__":
    app = NeedleApp()
    app.run(None)
