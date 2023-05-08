import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.css";
import CommunityList from "./renderer/views/community/list/CommunityList";
import CommunityNew from "./renderer/views/community/new/CommunityNew";
import CommunityDetail from "./renderer/views/community/detail/CommunityDetail";
import NavBar from "./renderer/components/NavBar";
import Login from "./renderer/views/authorization/login/Login";
import Register from "./renderer/views/authorization/register/Register";
import { Component } from 'react';
import AuthService from "./services/auth/AuthService";
import EventBus from "./utils/EventBus";

type Props = object;

type State = {
  userState: boolean;
}

class App extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.logOut = this.logOut.bind(this);

    this.state = {
      userState: false,
    };
  }

  componentDidMount() {
    const user = AuthService.getCurrentUser();

    if (user) {
      this.setState({
        userState: true,
      });
    }

    EventBus.on("logout", this.logOut);
  }

  componentWillUnmount() {
    EventBus.remove("logout", this.logOut);
  }

  logOut() {
    AuthService.logout();
    this.state = {
      userState: false,
    };
  }

  render() {
    const { userState } = this.state;

    return (
      <BrowserRouter>
        <div className="App">
          <NavBar userState={userState} onLogout={this.logOut} />
          <main>I like it</main>
          <Routes>
            <Route path="/login" element={<Login />} />
            <Route path="/register" element={<Register />} />
            <Route path="/" element={<CommunityList />} />
            <Route path="/community/new" element={<CommunityNew />} />
            <Route path="/community/detail/:id" element={<CommunityDetail />} />
          </Routes>
        </div>
      </BrowserRouter>
    );
  }
}

export default App;
