@react.component @genType
let make = () => {
  let current = Route.useRoute()
  <Recoil.RecoilRoot>
    {switch current {
    | Home => <Home />
    | Verify => <Verify />
    | DashBoard => <DashBoard />
    | _ => <NotFound />
    }}
  </Recoil.RecoilRoot>
}
