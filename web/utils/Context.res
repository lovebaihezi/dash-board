module AuthContext = {
  type auth = Passed(string) | NoAuth
  type verify = Pass((auth => auth) => unit) | No
  let context = React.createContext((NoAuth, No))
  module AuthProvider = {
    let provider = React.Context.provider(context)
    @react.component
    let make = (~value, ~children) => {
      React.createElement(provider, {"value": value, "children": children})
    }
  }
  let uesAuth = () => React.useContext(context)
}
