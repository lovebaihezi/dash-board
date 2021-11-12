let push = (~path=?, ()) => {
  switch path {
  | Some(path) => RescriptReactRouter.push(j`#/$path`)
  | None => RescriptReactRouter.push("")
  }
}

type routes =
  | Home
  | Verify
  | DashBoard
  | NotFound

let useRoute: unit => routes = () => {
  let {path} = RescriptReactRouter.useUrl()
  switch path {
  | list{} => Home
  | list{"verify"} => Verify
  | list{"dashboard", _} => DashBoard
  | _ => NotFound
  }
}
