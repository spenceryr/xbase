if exists("b:current_syntax")
  finish
endif

let s:cpo_save = &cpo
set cpo&vim


syn match   Operations   "\(Executing\|Compiling\|Processing\|Emitting\|Compiling\|Copying\|Validating\|Signing\|Linking\)"
syn match   Entitlement  "Entitlement"
syn match   Package      "Packaging"
syn region  Scope        display oneline start='^\[' end='\]'
syn region  LogSuccess   display oneline start='^\[Succeed\]$' end='$'
syn match   LogError     "^\(\[Error\]\)"
syn match   Target       "`\(\w.*\)`"
syn match   FilePath     "`\(\/.*\)`" 
syn region  Sep          display oneline start='-' end='-$'

hi def link Scope         Label
hi def link LogSuccess    healthSuccess
hi def link Operations    Function
hi def link Entitlement   Comment
hi def link Package       Comment
hi def link Sep           Comment
hi def link FilePath      String
hi def link Target        Label
hi def link LogError      Error

syn match HideAa "\`" conceal


let b:current_syntax = "xcodebuildlog"

let &cpo = s:cpo_save
unlet s:cpo_save