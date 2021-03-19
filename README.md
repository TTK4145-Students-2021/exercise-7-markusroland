Exercise 7: Phoenix
===================

*This exercise should be done (and handed in) individually*

Create a program (in any language, on any OS) that uses the *process pair* technique to print the numbers `1`, `2`, `3`, `4`, etc to a terminal window. The program should create its own backup: When the primary is running, *only the primary* should keep counting, and the backup should do nothing. When the primary dies, the backup should become the new primary, create its own new backup, and keep counting where the dead one left off. Make sure that no numbers are skipped!

[View a demo of process pairs in action (youtube)](https://youtu.be/HCgj9pqrTW4)

You cannot rely on the primary telling the backup when it has died (because it would have to be dead first...). Instead, have the primary broadcast that it is still alive, and have the backup become the primary when a certain number of messages have been missed.

You will need some form of communication between the primary and the backup. Some examples are:
 - Files: The primary writes to a file, and the backup reads it.
   - Either the time-stamp of the file or the contents can be used to detect if the primary is still alive.
   - This is usually the easiest way to do this exercise
 - Network: The easiest is to use UDP on `localhost`. TCP is also possible, but may be harder (since both endpoints need to be alive).
   - Use the ability to set a "read deadline" on the socket. You will either get a message (from the primary), or an error (indicating a timeout).
   - Remember to close the socket before creating your backup, so the next (newly created) backup can re-use the socket.
 - IPC, such as POSIX message queues: see [`msgget()` `msgsnd()` and `msgrcv()`](http://pubs.opengroup.org/onlinepubs/7990989775/xsh/sysmsg.h.html). With these you can create FIFO message queues.
 - [Signals](http://pubs.opengroup.org/onlinepubs/7990989775/xsh/signal.h.html): Use signals to interrupt other processes (You are already familiar with some of these, such as SIGSEGV (Segfault) and SIGTERM (Ctrl+C)). There are two custom signals you can use: SIGUSR1 and SIGUSR2. See `signal()`.
   - Note for D programmers: [SIGUSR is used by the GC.](http://dlang.org/phobos/core_memory.html)
 - Controlled shared memory: The system functions [`shmget()` and `shmat()`](http://pubs.opengroup.org/onlinepubs/7990989775/xsh/sysshm.h.html) let processes share memory.

You will also need to spawn the backup somehow. There should be a way to spawn processes or run shell commands in the standard library of your language of choice. The name of the terminal window is OS-dependent:
 - Ubuntu: `"gnome-terminal -x [commands]"`.  
   Alternatively, [use option `-e`](https://askubuntu.com/questions/1072688/what-is-the-difference-between-the-e-and-x-options-for-gnome-terminal).  
   Example: `"gnome-terminal -x ./pheonix"`.
 - Windows: The native shell on Windows is `cmd.exe`.
  - If you want to use `cmd`: Use `start [program_name]`.
    Example: `"start pheonix.exe"`, from whatever function spawns an OS process.  
    You may need to use `"start \"title\" call [args]"` for more complex commands.
  - If you want to use `powershell`: Use `start powershell [args]`.  
    Example (Dlang): `executeShell("start powershell rdmd pheonix.d");`.
 - OSX: `"osascript -e 'tell app \"Terminal\" to do script \"[terminal commands]\"'"`.  
   Note the quotes around `"Terminal"` and your commands. Be sure to escape these appropriately.
 
Some OS or Language-specific tips:
 - Linux: You can prevent a spawned terminal window from automatically closing by going to Edit -> Profile Preferences -> Title and Command -> When command exits. 
 - Golang: [`exec.Command` takes its arguments oddly](https://golang.org/pkg/os/exec/#Command), so you will have to separate out the program and its arguments. And use [backticks for raw strings](https://golang.org/ref/spec#String_literals).  
   Example (OSX): ``exec.Command("osascript", "-e", `tell app "Terminal" to do script "go run ` + filename + `.go"`)``
 - Windows: If you get the error `"start": executable file not found in %PATH%`, try inserting `"cmd /C start ...."`.  
   This seems to mostly apply to Go, so here's the full command to start your program in a PowerShell window:  
   `err := exec.Command("cmd", "/C", "start", "powershell", "go", "run", "pheonix.go").Run()`

Be careful! You don't want to create a [chain reaction](http://en.wikipedia.org/wiki/Fork_bomb)... If you do, you can use `pkill -f program_name` (Windows: `taskkill /F /IM program_name /T`) as a sledgehammer. Start with long periods and timeouts, so you have time to react if something goes wrong... And remember: *It's not murder if it's robots*.

Make sure you don't overcomplicate things. This program should be two loops (one to see if a primary exists, and one to do the work of the primary), with a call to create a new backup between them. You should not need any extra threads.

---

In case you want to use this on the project: Usually a program crashes for a reason. Restoring the program to the same state as it died in may cause it to crash in exactly the same way, all over again. How would you prevent this from happening?
