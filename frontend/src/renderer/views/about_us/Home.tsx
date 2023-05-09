import { Component } from "react";

import NavBar from "../../components/NavBar";
import AuthService from "../../../services/auth/AuthService";
import EventBus from "../../../utils/EventBus";
import { Route, Routes } from "react-router-dom";
import CommunityList from "../community/list/CommunityList";
import CommunityNew from "../community/new/CommunityNew";
import CommunityDetail from "../community/detail/CommunityDetail";


type Props = object;

type State = {
  userState: boolean;
  count: number;
};

class Home extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = {
      userState: false,
      count: 0,
    };
    this.logOut = this.logOut.bind(this);
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
    this.setState({
      userState: false,
    });
  }

  render() {
    const { userState } = this.state;

    return (
      <>
        <NavBar userState={userState} onLogout={this.logOut} />
     
        <div>
          <Routes>
          <Route path="/community/" element={<CommunityList />} />
            <Route path="/community/new" element={<CommunityNew />} />
            <Route path="/community/detail/:id" element={<CommunityDetail />} />
          </Routes>
        </div>
      </>
    );
  }
}

export default Home;
