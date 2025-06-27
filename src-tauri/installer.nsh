!macro customInstall
  Page custom AutoStartPage AutoStartPageLeave
!macroend

Var AutoStartCheckbox

Function AutoStartPage
  nsDialogs::Create 1018
  Pop $Dialog
  ${If} $Dialog == error
    Abort
  ${EndIf}
  ${NSD_CreateCheckbox} 0u 0u 100% 12u "开机启动"
  Pop $AutoStartCheckbox
  ${NSD_SetState} $AutoStartCheckbox 1 ; 默认勾选
  nsDialogs::Show
FunctionEnd

Function AutoStartPageLeave
  ${NSD_GetState} $AutoStartCheckbox $0
  StrCmp $0 1 0 +2
    ; 勾选了，写入一个标志文件
    FileOpen $1 "$INSTDIR\autostart.flag" w
    FileClose $1
FunctionEnd 