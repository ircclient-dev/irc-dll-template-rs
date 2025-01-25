; Call our function from the DLL
; Usage: /call_hello
alias call_hello {
  ; Call the "hello" function in "irc_dll_template_rs.dll" and save the result in the %result variable
  var %result = $dll(mirc_dll_template_rs.dll, hello, The quick brown fox jumped over the lazy dog.)
  ; Display the result in the active window
  echo -a Result from DLL: %result
  ; Since we have set m_keep to true, we need to manually unload the DLL.
  dll -u mirc_dll_template_rs.dll
}