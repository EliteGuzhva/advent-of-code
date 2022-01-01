" Config
set tabstop=4
set shiftwidth=4

" python configuration
let s:runner = "main"
let s:args = ""

" Key bindings
map <leader>c; :Run<CR>

" Launch runner
function! LaunchRunner()
  return ['cargo run', s:args]
endfunction

" Execute Run command in a vim-cmake console
function! ExecuteRun()
  let l:command = LaunchRunner()

  call cmake#console#SetCmdId('run')
  call cmake#command#Run(l:command, 0, 0)
endfunction

" Run
function! Run()
    call ExecuteRun()
endfunction

command! Run call Run()

