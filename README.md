Produces a panic is rust-analyzer with vscode on Windows.

Error message is

    thread '<unnamed>' panicked at 'index out of bounds: the len is 1 but the index is 1', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\chalk-ir-0.69.0\src\fold\subst.rs:58:19

Version info:

    DEBUG [7/20/2021, 3:38:58 PM]: c:\Users\snip\AppData\Roaming\Code\User\globalStorage\matklad.rust-analyzer\rust-analyzer-x86_64-pc-windows-msvc.exe --version: {
      status: 0,
      signal: null,
      output: [ null, 'rust-analyzer ea105f939 2021-07-19 stable\n', '' ],
      pid: 48896,
      stdout: 'rust-analyzer ea105f939 2021-07-19 stable\n',
      stderr: ''
    }


See [server_err.txt](./server_err.txt) for the full error (minus stack trace because I can't seem to make that work), [client_log.txt](./client_log.txt) and [server_log.txt](./server_log.txt) for log files.
