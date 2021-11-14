type shell
let bind: (Dom.element, ~font_size: int=?, ~font_family: string=?, unit) => shell
let write: (shell, string) => shell
let write_line: (shell, string, float) => shell
