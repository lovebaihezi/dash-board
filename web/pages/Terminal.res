@module external shell: {..} = "/web/styles/shell.module.css"
let shell = shell["default"]

@react.component @genType
let make = () => {
  let text_element = React.useRef(Js.Nullable.null)
  let cursor_element = React.useRef(Js.Nullable.null)
  let shell_ref = None->React.useRef
  React.useEffect0(() => {
    let s =
      10
      ->Js.Vector.make(
        10->Js.Vector.make("=><==><+><=>=>|=>|><|[]|||       ")->Js.Array2.joinWith(""),
      )
      ->Js.Array2.joinWith("")
    s->Js.String2.split("\n")->Js.log
    shell_ref.current =
      text_element.current
      ->Js.Nullable.toOption
      ->Belt.Option.map(v => {
        Shell.bind(v, ())
      })
      ->Belt.Option.map(value => value->Shell.write(s))
    Some(() => ())
  })
  <div className={shell["shell-root"]}>
    <canvas className={shell["shell-terminal"]} ref={text_element->ReactDOM.Ref.domRef} />
    <canvas className={shell["shell-cursor"]} ref={cursor_element->ReactDOM.Ref.domRef} />
  </div>
}
