@react.component @genType
let make = () => {
  let (permission, _changePermission) = Recoil.useRecoilState(Permission.state)
  React.string("none")
}
