@react.component @genType
let make = () => {
  let current = Route.useRoute()
  <Recoil.RecoilRoot>
    {switch current {
    | Home => <Terminal />
    | Verify => <Verify />
    | DashBoard => <DashBoard />
    | _ => <NotFound />
    }}
  </Recoil.RecoilRoot>
}
