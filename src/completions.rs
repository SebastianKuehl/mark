use clap::CommandFactory;
use clap_complete::{generate, Shell};

use crate::cli::Cli;

const MARK_BASH_OPTS: &str =
    "-s -r -V -h --cleanup --no-open --single --recursive --theme --version --help";
const MARK_BASH_OPTS_WITH_SUBCOMMANDS: &str =
    "-s -r -V -h --cleanup --no-open --single --recursive --theme --version --help [FILE] completions config cleanup-home help";
const BASH_REGISTRATION_MARKER: &str = "\nif [[ \"${BASH_VERSINFO[0]}\"";
const BASH_COMPGEN_LINE: &str = "            COMPREPLY=( $(compgen -W \"${opts}\" -- \"${cur}\") )";
const BASH_FILE_GUARD: &str = r#"
_mark_has_top_level_file() {
    local idx word expecting_value=0
    for ((idx=1; idx<COMP_CWORD; idx++)); do
        word="${COMP_WORDS[idx]}"
        if [[ ${expecting_value} -eq 1 ]]; then
            expecting_value=0
            continue
        fi
        case "${word}" in
            --theme)
                expecting_value=1
                ;;
            -*)
                ;;
            *)
                return 0
                ;;
        esac
    done
    return 1
}
"#;

pub fn render(shell: Shell) -> String {
    let mut cmd = Cli::command();
    let mut buf: Vec<u8> = Vec::new();
    generate(shell, &mut cmd, "mark", &mut buf);

    let script = String::from_utf8(buf).expect("completion output should be valid UTF-8");
    if shell == Shell::Bash {
        patch_bash_completion(script)
    } else {
        script
    }
}

fn patch_bash_completion(script: String) -> String {
    let opts_pos = script
        .find(MARK_BASH_OPTS_WITH_SUBCOMMANDS)
        .expect("bash completion script must contain top-level mark options");
    let next_case_pos = script[opts_pos..]
        .find("        mark__cleanup__home)")
        .expect("bash completion script must contain cleanup-home case")
        + opts_pos;
    let mark_case = &script[opts_pos..next_case_pos];
    let second_compgen = mark_case
        .rfind(BASH_COMPGEN_LINE)
        .expect("bash completion script must contain fallback top-level completion branch")
        + opts_pos;

    let mut patched = script.replacen(
        BASH_REGISTRATION_MARKER,
        &format!("{BASH_FILE_GUARD}{BASH_REGISTRATION_MARKER}"),
        1,
    );
    patched.replace_range(
        second_compgen..second_compgen + BASH_COMPGEN_LINE.len(),
        &format!(
            "            if _mark_has_top_level_file; then\n                COMPREPLY=( $(compgen -W \"{MARK_BASH_OPTS}\" -- \"${{cur}}\") )\n            else\n                COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )\n            fi"
        ),
    );
    patched
}
