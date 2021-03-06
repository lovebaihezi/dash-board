type shell = {
  ctx: Webapi.Canvas.Canvas2d.t,
  text: array<string>,
  current_width: float,
  current_height: float,
  cursor_position: (float, float),
  window_width: int,
  window_height: int,
  font_size: int,
  font_family: string,
}

let dbg = (arg: 'a) => {
  Js.log(arg)
  arg
}

let bind = (dom, ~font_size=14, ~font_family="fira code", ()) => {
  open Webapi.Dom
  open Webapi.Canvas.Canvas2d
  let (window_width, window_height) = (window->Window.innerWidth, window->Window.innerHeight)
  let ctx = {
    open Webapi.Canvas
    dom->CanvasElement.setWidth(window_width)
    dom->CanvasElement.setHeight(window_height)
    dom->CanvasElement.getContext2d
  }
  ctx->font(`${font_size->Belt.Int.toString}px ${font_family}`)
  {
    ctx: ctx,
    text: [],
    window_width: window_width,
    window_height: window_height,
    current_width: 0.,
    current_height: font_size->Belt.Int.toFloat,
    cursor_position: (0., 0.),
    font_size: font_size,
    font_family: font_family,
  }
}

/*
 * make long line to fitful width
 */
let line_split = (line: string, max_width: float, ctx: shell): array<(float, string)> => {
  open Webapi.Canvas.Canvas2d

  line->Js.String.castToArrayLike->from->reduce((prev, v) => {
    pop(prev)
    ->Belt.Option.map(last => {
      let (last_width, s) = last
      let width = v->measureText(ctx.ctx)->width
      prev->concat(
        if last_width +. width > max_width {
          [(last_width, s), (width, v)]
        } else {
          [(last_width +. width, `${s}${v}`)]
        },
      )
    })
    ->Belt.Option.getExn
  }, [(0., "")])
}

// infinite recursion
let rec write_line = (line: string, line_width: float, ctx: shell) => {
  open Webapi.Canvas.Canvas2d
  open Js.Array2
  // BUG: infinity recursion call
  if dbg(ctx.current_width +. line_width < ctx.window_width->Belt.Int.toFloat) {
    line->fillText(~x=ctx.current_width, ~y=ctx.current_height, ctx.ctx)
    let (new_width, new_height) = if (
      ctx.current_width +. line_width < ctx.window_width->Belt.Int.toFloat
    ) {
      (ctx.current_width +. line_width, ctx.current_height)
    } else {
      (0., ctx.current_height +. ctx.font_size->Belt.Int.toFloat)
    }
    {
      ...ctx,
      current_width: new_width,
      current_height: new_height,
    }
  } else {
    line->line_split(ctx.window_width->Belt.Int.toFloat, ctx)->reduce((ctx, v) => {
      let (width, line) = dbg(v)
      write_line(line, width, ctx)
    }, ctx)
  }
}

/*
 * write to shell
 * string will split with "\n"
 * then measure each unicode draw length
 * then reduce them to the fitful width
 */
let write = (shell, code) => {
  open Webapi.Canvas
  open Js
  open Array2
  code
  ->String2.split("\n")
  ->map(line => (line, line->Canvas2d.measureText(shell.ctx)->Canvas2d.width))
  ->reduce((ctx, (line, width)) => {
    line->write_line(width, ctx)
  }, shell)
}

let delete = () => {()}

let unbind: Dom.htmlCanvasElement => unit = dom => {()}

let rewrite_history: shell => shell = shell => {
  shell
}
