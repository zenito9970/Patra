_patra() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            patra)
                cmd="patra"
                ;;
            
            *)
                ;;
        esac
    done

    case "${cmd}" in
        patra)
            opts=" -h -V -i -o  --help --version --in-dir --out-dir --threads --logfile  <command> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --in-dir)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                    -i)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                --out-dir)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                    -o)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                --threads)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                --logfile)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        
    esac
}

complete -F _patra -o bashdefault -o default patra
