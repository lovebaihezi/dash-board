@react.component @genType
let make = (~to: string, ~children: React.element, ()) => {
  <a
    onClick={e => {
      e->ReactEvent.Mouse.preventDefault
      to->RescriptReactRouter.push
    }}
    href=to>
    children
  </a>
}
