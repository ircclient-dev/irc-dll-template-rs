; This is the entry point for the debugger script, and will be called on each launch.
on 1:SIGNAL:debug_script_loaded:{
  echo -at Result from our Rust DLL: $dll.call(hello, $null)
}

; DON'T EDIT BELOW THIS LINE UNLESS YOU KNOW WHAT YOU'RE DOING!

; The following aliases are used to load and call functions from the DLL. There are a few things to note:
; - The DLL must be built with the correct target for the current instance of mIRC/AdiIRC.
; - The $dll.call alias will attempt to call each possible DLL target until one is successful.
; - The DLL will be unloaded on the next tick after calling a function.

; Usage $dll.call(<function>, <parameters>) - Calls the specified function in the DLL, and unloads the DLL on next tick.
alias -l dll.call {
  var %i = 0
  :loop
  ; Native DLL (no target specified)
  if (%i == 0) {
    var %result = $dll($dll.filename, $1, $2-)
    .timer -m 1 1 $dll.unload()
    return %result
  }
  ; DLL built with target specified
  elseif (%i <= $numtok($dll.targets, 32)) {
    var %result = $dll($dll.filename($gettok($dll.targets, %i, 32)), $1, $2-)
    .timer -m 1 1 $dll.unload($gettok($dll.targets, %i, 32))
    return %result
  }
  else {
    echo $color(info) -at * Unable to load DLL. Please ensure the DLL is built for the correct target:
    var %i = 1
    while (%i <= $numtok($dll.targets, 32)) {
      var %target = $gettok($dll.targets, %i, 32)
      echo $color(info) -at * For %target $+ : $ cargo build --target= $+ %target
      inc %i
    }
    halt
  }
  goto finish | :error | reseterror | inc %i | goto loop | :finish
}

; Usage $dll.filename(<target>) - Returns the filename of the DLL to load.
alias -l dll.filename return $qt($scriptdir $+ target $+ $iif($1-, \ $+ $v1) $+ \debug\irc_dll_template_rs.dll)

; Usage $dll.targets - Returns the possible build targets for the DLL, based on the bitness of the current instance.
alias -l dll.targets return $eval($ $+ dll.targets. $+ $bits, 2)
; Note: We try aarch64 first, because aarch64 systems often support x86_64 as well (and not vice versa).
; - I'm doubtful that an aarch64 executable would be able to run the x86_64 DLL, but i'll leave it here until I know better.
alias -l dll.targets.64 return aarch64-pc-windows-msvc aarch64-pc-windows-gnu x86_64-pc-windows-msvc x86_64-pc-windows-gnu
alias -l dll.targets.32 return i686-pc-windows-msvc i686-pc-windows-gnu

; Usage: .timer -m 1 1 $dll.unload(<target>) - Unloads the DLL on the next tick.
; Note: We can't call /dll.unload directly because the script may be unloaded before the timer fires (eg. When a breakpoint is called).
alias -l dll.unload return dll -u $dll.filename($1-)