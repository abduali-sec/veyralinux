export PS1='\[\e[1;37m\][\u@\h \W]\$\[\e[0m\] '

alias vinfo='veyra-info'
alias vsettings='veyra-settings'
alias vdisks='veyra-disks'
alias vinstall='veyra-installer'

if command -v fastfetch >/dev/null 2>&1; then
    fastfetch --config /etc/fastfetch/config.jsonc
fi

alias v='veyra'
alias vi='veyra installer'
alias vs='veyra settings'
alias vu='veyra update'
alias va='veyra about'

alias vc='veyra-center'

# Veyra shortcuts
alias vinfo='veyra info'
alias vcenter='veyra center'
alias vsettings='veyra settings'
alias vdoctor='veyra doctor'
alias vupdate='veyra update'
alias vhardware='veyra hardware'
alias vaudio='veyra audio'
alias vprint='veyra print'
alias vpower='veyra power'
alias vfirewall='veyra firewall'
alias vinstall='sudo veyra-installer'
alias vpkg='vpm'
