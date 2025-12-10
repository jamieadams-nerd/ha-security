
==================================================
Concept overview

You can have multiple GTK4 applications that:
	•	run perfectly fine on their own (standalone mode), AND
	•	can also be launched from a single “toolbox” application that uses a GTK4 GridView.

Good news:
	•	The standalone applications do NOT need any special coding to support this.
	•	The toolbox simply launches them as separate processes.
	•	Each tool keeps its own Gtk.Application instance, window lifecycle, and audit boundaries.

This is actually the preferred model for high-assurance systems:
	•	clean isolation
	•	predictable behavior
	•	simpler auditing
	•	fewer GTK interdependencies

==================================================
Files in this example

Three Python scripts:
	1.	toolbox.py
A GTK4 GridView-based launcher (modern replacement for IconView)
	2.	script_one.py
A standalone GTK4 application
	3.	script_two.py
Another standalone GTK4 application

You can run any of them directly, or launch script_one.py and script_two.py from toolbox.py.


