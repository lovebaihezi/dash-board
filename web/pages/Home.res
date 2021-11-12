@react.component @genType
let make = () => {
  let (permission, _changePermission) = Recoil.useRecoilState(Permission.state)
  let v = switch permission {
  | Some(v) => v
  | None => "None"
  }->React.string
  v
}
