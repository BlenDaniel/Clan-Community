import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.css";
import Login from "./renderer/views/authorization/login/Login";
import Register from "./renderer/views/authorization/register/Register";
import { Component } from "react";
import Home from "./renderer/views/home/Home";
import Index from "./renderer/views/index/Index";
import About from "./renderer/views/aboutus/About";
import Contact from "./renderer/views/contactus/Contact";

type Props = object;

type State = {
  userState: boolean;
};

class App extends Component<Props, State> {
  render() {
    return (
      <BrowserRouter>
        <div className="App">
          <Routes>
          <Route path="/" element={<Index />} />
          <Route path="/home/*" element={<Home />} />
          <Route path="/contact" element={<Contact />} />
            <Route path="/about" element={<About />} />
            <Route path="/login" element={<Login />} />
            <Route path="/register" element={<Register />} />
          </Routes>
        </div>
      </BrowserRouter>
    );
  }
}

export default App;
