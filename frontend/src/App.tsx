import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.css";
import CommunityList from "./renderer/views/community/list/CommunityList";
import CommunityNew from "./renderer/views/community/new/CommunityNew";
import CommunityDetail from "./renderer/views/community/detail/CommunityDetail";
import Navbar from "./renderer/components/Navbar";

function App() {
  return (
    <>
      <Navbar />
      <BrowserRouter>
        <div className="App">
          <Routes>
            <Route path="/" element={<CommunityList />} />
            <Route path="/community/new" element={<CommunityNew />} />
            <Route path="/community/detail/:id" element={<CommunityDetail />} />
          </Routes>
        </div>
      </BrowserRouter>
    </>
  );
}

export default App;
